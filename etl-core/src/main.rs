use arrow::array::{ArrayRef, BooleanArray, Float64Array, Int64Array, StringArray, UInt64Array, Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use reqwest;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct ApiToArrowConverter {
    client: reqwest::Client,
}

impl ApiToArrowConverter {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Fetch JSON from API and convert to Arrow RecordBatch
    pub async fn api_to_arrow(&self, url: &str) -> Result<RecordBatch, Box<dyn std::error::Error>> {
        // Fetch JSON data
        let json_data = self.fetch_json_data(url).await?;
        
        // Extract records from the response (assuming they're in a "results" array)
        let records = self.extract_records(&json_data)?;
        
        // Convert to Arrow
        self.json_to_arrow(&records)
    }

    /// Fetch JSON data from URL
    async fn fetch_json_data(&self, url: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let response = self.client.get(url).send().await?;
        let json: Value = response.json().await?;
        Ok(json)
    }

    /// Extract records array from API response
    fn extract_records(&self, json_data: &Value) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        // Handle common API response patterns
        if let Some(results) = json_data.get("results") {
            if let Value::Array(records) = results {
                return Ok(records.clone());
            }
        }
        
        // If no "results" field, try direct array
        if let Value::Array(records) = json_data {
            return Ok(records.clone());
        }
        
        // Try "data" field
        if let Some(data) = json_data.get("data") {
            if let Value::Array(records) = data {
                return Ok(records.clone());
            }
        }
        
        Err("Could not extract records from API response".into())
    }

    /// Convert JSON array to Arrow RecordBatch
    fn json_to_arrow(&self, json_values: &[Value]) -> Result<RecordBatch, Box<dyn std::error::Error>> {
        if json_values.is_empty() {
            return Err("Empty JSON array".into());
        }

        // Infer schema from JSON data
        let schema = self.infer_schema(json_values)?;
        
        // Convert JSON values to Arrow arrays
        let arrays = self.json_to_arrays(json_values, &schema)?;
        
        // Create RecordBatch
        let record_batch = RecordBatch::try_new(
            Arc::new(schema),
            arrays,
        )?;
        
        Ok(record_batch)
    }

    /// Infer schema from JSON objects
    fn infer_schema(&self, json_values: &[Value]) -> Result<Schema, Box<dyn std::error::Error>> {
        let mut field_stats: HashMap<String, FieldStats> = HashMap::new();

        // Collect all possible fields and their types
        for value in json_values {
            if let Value::Object(obj) = value {
                for (key, val) in obj {
                    let stats = field_stats.entry(key.clone()).or_insert_with(FieldStats::new);
                    stats.update(val);
                }
            }
        }

        // Create fields
        let mut fields = Vec::new();
        for (field_name, stats) in field_stats {
            let data_type = stats.determine_type();
            let nullable = stats.null_count > 0 || stats.total_count == 0;
            fields.push(Field::new(field_name, data_type, nullable));
        }

        Ok(Schema::new(fields))
    }

    /// Convert JSON values to Arrow arrays based on schema
    fn json_to_arrays(&self, json_values: &[Value], schema: &Schema) -> Result<Vec<ArrayRef>, Box<dyn std::error::Error>> {
        let mut arrays = Vec::new();

        for field in schema.fields() {
            let array = self.create_array_for_field(json_values, field)?;
            arrays.push(array);
        }

        Ok(arrays)
    }

    /// Create Arrow array for a specific field
    fn create_array_for_field(&self, json_values: &[Value], field: &Field) -> Result<ArrayRef, Box<dyn std::error::Error>> {
        match field.data_type() {
            DataType::Boolean => self.build_boolean_array(json_values, field.name()),
            DataType::Int64 => self.build_int64_array(json_values, field.name()),
            DataType::UInt64 => self.build_uint64_array(json_values, field.name()),
            DataType::Float64 => self.build_float64_array(json_values, field.name()),
            DataType::Utf8 => self.build_string_array(json_values, field.name()),
            _ => self.build_string_array(json_values, field.name()), // Fallback to string
        }
    }

    fn build_boolean_array(&self, json_values: &[Value], field_name: &str) -> Result<ArrayRef, Box<dyn std::error::Error>> {
        let mut builder = arrow::array::BooleanBuilder::with_capacity(json_values.len());
        for value in json_values {
            if let Value::Object(obj) = value {
                match obj.get(field_name) {
                    Some(Value::Bool(b)) => builder.append_value(*b),
                    Some(Value::String(s)) => {
                        match s.to_lowercase().as_str() {
                            "true" | "1" => builder.append_value(true),
                            "false" | "0" => builder.append_value(false),
                            _ => builder.append_null(),
                        }
                    }
                    Some(Value::Number(n)) => {
                        if let Some(i) = n.as_i64() {
                            builder.append_value(i != 0);
                        } else if let Some(f) = n.as_f64() {
                            builder.append_value(f != 0.0);
                        } else {
                            builder.append_null();
                        }
                    }
                    Some(Value::Null) | None => builder.append_null(),
                    _ => builder.append_null(),
                }
            } else {
                builder.append_null();
            }
        }
        Ok(Arc::new(builder.finish()))
    }

    fn build_int64_array(&self, json_values: &[Value], field_name: &str) -> Result<ArrayRef, Box<dyn std::error::Error>> {
        let mut builder = arrow::array::Int64Builder::with_capacity(json_values.len());
        for value in json_values {
            if let Value::Object(obj) = value {
                match obj.get(field_name) {
                    Some(Value::Number(n)) => {
                        if let Some(i) = n.as_i64() {
                            builder.append_value(i);
                        } else if let Some(u) = n.as_u64() {
                            builder.append_value(u as i64);
                        } else if let Some(f) = n.as_f64() {
                            builder.append_value(f as i64);
                        } else {
                            builder.append_null();
                        }
                    }
                    Some(Value::String(s)) => {
                        match s.parse::<i64>() {
                            Ok(i) => builder.append_value(i),
                            Err(_) => builder.append_null(),
                        }
                    }
                    Some(Value::Null) | None => builder.append_null(),
                    _ => builder.append_null(),
                }
            } else {
                builder.append_null();
            }
        }
        Ok(Arc::new(builder.finish()))
    }

    fn build_uint64_array(&self, json_values: &[Value], field_name: &str) -> Result<ArrayRef, Box<dyn std::error::Error>> {
        let mut builder = arrow::array::UInt64Builder::with_capacity(json_values.len());
        for value in json_values {
            if let Value::Object(obj) = value {
                match obj.get(field_name) {
                    Some(Value::Number(n)) => {
                        if let Some(u) = n.as_u64() {
                            builder.append_value(u);
                        } else if let Some(i) = n.as_i64() {
                            if i >= 0 {
                                builder.append_value(i as u64);
                            } else {
                                builder.append_null();
                            }
                        } else {
                            builder.append_null();
                        }
                    }
                    Some(Value::String(s)) => {
                        match s.parse::<u64>() {
                            Ok(u) => builder.append_value(u),
                            Err(_) => builder.append_null(),
                        }
                    }
                    Some(Value::Null) | None => builder.append_null(),
                    _ => builder.append_null(),
                }
            } else {
                builder.append_null();
            }
        }
        Ok(Arc::new(builder.finish()))
    }

    fn build_float64_array(&self, json_values: &[Value], field_name: &str) -> Result<ArrayRef, Box<dyn std::error::Error>> {
        let mut builder = arrow::array::Float64Builder::with_capacity(json_values.len());
        for value in json_values {
            if let Value::Object(obj) = value {
                match obj.get(field_name) {
                    Some(Value::Number(n)) => {
                        if let Some(f) = n.as_f64() {
                            builder.append_value(f);
                        } else if let Some(i) = n.as_i64() {
                            builder.append_value(i as f64);
                        } else if let Some(u) = n.as_u64() {
                            builder.append_value(u as f64);
                        } else {
                            builder.append_null();
                        }
                    }
                    Some(Value::String(s)) => {
                        match s.parse::<f64>() {
                            Ok(f) => builder.append_value(f),
                            Err(_) => builder.append_null(),
                        }
                    }
                    Some(Value::Null) | None => builder.append_null(),
                    _ => builder.append_null(),
                }
            } else {
                builder.append_null();
            }
        }
        Ok(Arc::new(builder.finish()))
    }

    fn build_string_array(&self, json_values: &[Value], field_name: &str) -> Result<ArrayRef, Box<dyn std::error::Error>> {
        let mut builder = arrow::array::StringBuilder::with_capacity(json_values.len(), 1024);
        for value in json_values {
            if let Value::Object(obj) = value {
                match obj.get(field_name) {
                    Some(Value::String(s)) => builder.append_value(s),
                    Some(Value::Null) | None => builder.append_null(),
                    Some(other) => builder.append_value(&other.to_string()),
                }
            } else {
                builder.append_null();
            }
        }
        Ok(Arc::new(builder.finish()))
    }
}

#[derive(Debug)]
struct FieldStats {
    bool_count: usize,
    int_count: usize,
    uint_count: usize,
    float_count: usize,
    string_count: usize,
    null_count: usize,
    total_count: usize,
}

impl FieldStats {
    fn new() -> Self {
        Self {
            bool_count: 0,
            int_count: 0,
            uint_count: 0,
            float_count: 0,
            string_count: 0,
            null_count: 0,
            total_count: 0,
        }
    }

    fn update(&mut self, value: &Value) {
        self.total_count += 1;
        match value {
            Value::Null => self.null_count += 1,
            Value::Bool(_) => self.bool_count += 1,
            Value::Number(n) => {
                if n.is_i64() {
                    self.int_count += 1;
                } else if n.is_u64() {
                    self.uint_count += 1;
                } else {
                    self.float_count += 1;
                }
            }
            Value::String(_) => self.string_count += 1,
            Value::Array(_) | Value::Object(_) => self.string_count += 1,
        }
    }

    fn determine_type(&self) -> DataType {
        let non_null_count = self.total_count.saturating_sub(self.null_count);
        if non_null_count == 0 {
            return DataType::Utf8; // Default for all-null fields
        }

        // Determine primary type based on counts
        let counts = [
            (self.float_count, DataType::Float64),
            (self.int_count, DataType::Int64),
            (self.uint_count, DataType::UInt64),
            (self.bool_count, DataType::Boolean),
            (self.string_count, DataType::Utf8),
        ];

        counts.iter()
            .max_by_key(|(count, _)| count)
            .map(|(_, data_type)| data_type.clone())
            .unwrap_or(DataType::Utf8)
    }
}

// Usage example
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let converter = ApiToArrowConverter::new();
    
    // Fetch and convert the SpaceFlight News API data
    let record_batch = converter
        .api_to_arrow("https://api.spaceflightnewsapi.net/v4/articles/")
        .await?;
    println!("RecordBatch created from SpaceFlight News API: {:#?}", record_batch);
    println!("Created RecordBatch with {} rows and {} columns", 
             record_batch.num_rows(), 
             record_batch.num_columns());
    
    println!("Schema: {:#?}", record_batch.schema());
    
    // Print first few rows as example
    if record_batch.num_rows() > 0 {
        println!("\nFirst 3 rows:");
        let max_rows = std::cmp::min(3, record_batch.num_rows());
        for i in 0..max_rows {
            println!("Row {}:", i);
            for (j, field) in record_batch.schema().fields().iter().enumerate() {
                let column = record_batch.column(j);
                match field.data_type() {
                    DataType::Utf8 => {
                        let string_array = column.as_any().downcast_ref::<StringArray>().unwrap();
                        if string_array.is_null(i) {
                            println!("  {}: null", field.name());
                        } else {
                            println!("  {}: {}", field.name(), string_array.value(i));
                        }
                    }
                    DataType::Int64 => {
                        let int_array = column.as_any().downcast_ref::<Int64Array>().unwrap();
                        if int_array.is_null(i) {
                            println!("  {}: null", field.name());
                        } else {
                            println!("  {}: {}", field.name(), int_array.value(i));
                        }
                    }
                    DataType::Float64 => {
                        let float_array = column.as_any().downcast_ref::<Float64Array>().unwrap();
                        if float_array.is_null(i) {
                            println!("  {}: null", field.name());
                        } else {
                            println!("  {}: {}", field.name(), float_array.value(i));
                        }
                    }
                    DataType::Boolean => {
                        let bool_array = column.as_any().downcast_ref::<BooleanArray>().unwrap();
                        if bool_array.is_null(i) {
                            println!("  {}: null", field.name());
                        } else {
                            println!("  {}: {}", field.name(), bool_array.value(i));
                        }
                    }
                    _ => {
                        println!("  {}: (unsupported type)", field.name());
                    }
                }
            }
            println!();
        }
    }
    
    Ok(())
}
