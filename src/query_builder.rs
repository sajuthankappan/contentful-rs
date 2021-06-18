use std::collections::HashMap;

pub struct QueryBuilder {
    query_string_values: HashMap<String, String>,
}

impl QueryBuilder {
    pub fn new() -> QueryBuilder {
        let query_string_values = HashMap::new();

        QueryBuilder {
            query_string_values,
        }
    }

    pub fn content_type_is<S>(mut self, content_type_id: S) -> QueryBuilder
    where
        S: Into<String>,
    {
        self.query_string_values
            .insert("content_type".into(), content_type_id.into());
        self
    }

    pub fn order_by<S>(mut self, order: S) -> QueryBuilder
    where
        S: Into<String>,
    {
        self.query_string_values
            .insert("order".into(), order.into());
        self
    }

    pub fn limit(mut self, limit: i32) -> QueryBuilder {
        self.query_string_values
            .insert("order".to_string(), limit.to_string());
        self
    }

    pub fn skip(mut self, skip: i32) -> QueryBuilder {
        self.query_string_values
            .insert("skip".to_string(), skip.to_string());
        self
    }

    pub fn include(mut self, level: i32) -> QueryBuilder {
        self.query_string_values
            .insert("include".into(), level.to_string());
        self
    }

    pub fn locale_is<S>(mut self, locale: S, value: S) -> QueryBuilder
    where
        S: Into<String>,
    {
        self.query_string_values.insert(locale.into(), value.into());
        self
    }

    pub fn field_equals<S>(mut self, field: S, value: S) -> QueryBuilder
    where
        S: Into<String>,
    {
        self.query_string_values.insert(field.into(), value.into());
        self
    }

    pub fn field_does_not_equal<S>(mut self, field: S, value: S) -> QueryBuilder
    where
        S: Into<String>,
    {
        self = self.add_field_restriction(field, value, "[ne]");
        self
    }

    pub fn field_equals_all<S>(mut self, field: S, values: S) -> QueryBuilder
    where
        S: Into<String>,
    {
        self = self.add_field_restriction(field, values, "[all]");
        self
    }

    pub fn field_includes<S>(mut self, field: S, values: S) -> QueryBuilder
    where
        S: Into<String>,
    {
        self = self.add_field_restriction(field, values, "[in]");
        self
    }

    pub fn field_excludes<S>(mut self, field: S, values: S) -> QueryBuilder
    where
        S: Into<String>,
    {
        self = self.add_field_restriction(field, values, "[nin]");
        self
    }

    pub fn field_exists<S>(mut self, field: S, must_exist: bool) -> QueryBuilder
    where
        S: Into<String>,
    {
        let key = format!("{}[exists]", field.into());
        self.query_string_values.insert(key, must_exist.to_string());
        self
    }

    pub fn field_less_than<S>(mut self, field: S, value: S) -> QueryBuilder
    where
        S: Into<String>,
    {
        self = self.add_field_restriction(field, value, "[lt]");
        self
    }

    pub fn field_less_than_or_equal_to<S>(mut self, field: S, value: S) -> QueryBuilder
    where
        S: Into<String>,
    {
        self = self.add_field_restriction(field, value, "[lte]");
        self
    }

    pub fn field_greater_than<S>(mut self, field: S, value: S) -> QueryBuilder
    where
        S: Into<String>,
    {
        self = self.add_field_restriction(field, value, "[gt]");
        self
    }

    pub fn field_greater_than_or_equal_to<S>(mut self, field: S, value: S) -> QueryBuilder
    where
        S: Into<String>,
    {
        self = self.add_field_restriction(field, value, "[gte]");
        self
    }

    pub fn field_matches<S>(mut self, field: S, value: S) -> QueryBuilder
    where
        S: Into<String>,
    {
        self = self.add_field_restriction(field, value, "[match]");
        self
    }

    pub fn links_to_entry<S>(mut self, id: S) -> QueryBuilder
    where
        S: Into<String>,
    {
        self = self.add_field_restriction("links_to_entry".into(), id.into(), "");
        self
    }

    pub fn links_to_asset<S>(mut self, id: S) -> QueryBuilder
    where
        S: Into<String>,
    {
        self = self.add_field_restriction("links_to_asset".into(), id.into(), "");
        self
    }

    pub fn build(&self) -> String {
        let mut query_string = String::new();
        let mut has_query = false;
        for (query_key, query_value) in &self.query_string_values {
            if has_query {
                query_string.push_str("&");
            } else {
                query_string.push_str("?");
            }
            query_string.push_str(query_key);
            query_string.push_str("=");
            query_string.push_str(query_value.as_str());
            has_query = true;
        }

        query_string
    }

    pub fn add_field_restriction<S>(mut self, field: S, value: S, operator: &str) -> QueryBuilder
    where
        S: Into<String>,
    {
        let key = format!("{}{}", field.into(), operator);
        self.query_string_values.insert(key, value.into());
        self
    }
}
