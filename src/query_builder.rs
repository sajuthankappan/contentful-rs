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

    pub fn content_type_is(mut self, content_type_id: &str) -> QueryBuilder {
        self.query_string_values
            .insert("content_type".to_string(), content_type_id.to_string());
        self
    }

    pub fn order_by(mut self, order: &str) -> QueryBuilder {
        self.query_string_values
            .insert("order".to_string(), order.to_string());
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
            .insert("include".to_string(), level.to_string());
        self
    }

    pub fn locale_is(mut self, locale: &str, value: &str) -> QueryBuilder {
        self.query_string_values
            .insert(locale.to_string(), value.to_string());
        self
    }

    pub fn field_equals(mut self, field: &str, value: &str) -> QueryBuilder {
        self.query_string_values
            .insert(field.to_string(), value.to_string());
        self
    }

    pub fn field_does_not_equal(mut self, field: &str, value: &str) -> QueryBuilder {
        self = self.add_field_restriction(field, value, "[ne]");
        self
    }

    pub fn field_equals_all(mut self, field: &str, values: &str) -> QueryBuilder {
        self = self.add_field_restriction(field, values, "[all]");
        self
    }

    pub fn field_includes(mut self, field: &str, values: &str) -> QueryBuilder {
        self = self.add_field_restriction(field, values, "[in]");
        self
    }

    pub fn field_excludes(mut self, field: &str, values: &str) -> QueryBuilder {
        self = self.add_field_restriction(field, values, "[nin]");
        self
    }

    pub fn field_exists(mut self, field: &str, must_exist: bool) -> QueryBuilder {
        let key = format!("{}[exists]", field);
        self.query_string_values.insert(key, must_exist.to_string());
        self
    }

    pub fn field_less_than(mut self, field: &str, value: &str) -> QueryBuilder {
        self = self.add_field_restriction(field, value, "[lt]");
        self
    }

    pub fn field_less_than_or_equal_to(mut self, field: &str, value: &str) -> QueryBuilder {
        self = self.add_field_restriction(field, value, "[lte]");
        self
    }

    pub fn field_greater_than(mut self, field: &str, value: &str) -> QueryBuilder {
        self = self.add_field_restriction(field, value, "[gt]");
        self
    }

    pub fn field_greater_than_or_equal_to(mut self, field: &str, value: &str) -> QueryBuilder {
        self = self.add_field_restriction(field, value, "[gte]");
        self
    }

    pub fn field_matches(mut self, field: &str, value: &str) -> QueryBuilder {
        self = self.add_field_restriction(field, value, "[match]");
        self
    }

    pub fn links_to_entry(mut self, id: &str) -> QueryBuilder {
        self = self.add_field_restriction("links_to_entry", id, "");
        self
    }

    pub fn links_to_asset(mut self, id: &str) -> QueryBuilder {
        self = self.add_field_restriction("links_to_asset", id, "");
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

    pub fn add_field_restriction(
        mut self,
        field: &str,
        value: &str,
        operator: &str,
    ) -> QueryBuilder {
        let key = format!("{}{}", field, operator);
        self.query_string_values.insert(key, value.into());
        self
    }
}
