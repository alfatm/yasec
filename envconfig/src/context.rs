pub struct Context<T> {
    prefix: Vec<String>,
    var_name: String,
    default_var_value: Option<T>,
}

impl<T> Context<T> {
    pub fn new(prefix: impl AsRef<str>) -> Self {
        let prefix = prefix.as_ref().to_owned();
        let mut ret = Self {
            prefix: vec![],
            var_name: String::default(),
            default_var_value: None,
        };
        if prefix.len() > 0 {
            ret.prefix.push(prefix);
        }
        ret
    }

    pub fn prefix(&self) -> String {
        self.prefix.join("_")
    }

    pub fn with_var_name(&self, var_name: &str) -> Self {
        Self {
            prefix: self.prefix.clone(),
            var_name: var_name.to_owned(),
            default_var_value: None,
        }
    }

    pub fn infer_var_name(&self) -> String {
        match self.var_name.len() {
            0 => self.prefix(),
            _ => self.var_name.to_owned(),
        }
    }

    pub fn push_prefix(&self, val: String) -> Self {
        let mut ret = Self {
            prefix: self.prefix.clone(),
            var_name: self.var_name.clone(),
            default_var_value: None,
        };
        ret.prefix.push(val);
        ret
    }

    pub fn with_default_var_value<U>(&self, val: Option<U>) -> Context<U> {
        Context {
            prefix: self.prefix.clone(),
            var_name: self.var_name.clone(),
            default_var_value: val,
        }
    }

    pub fn take_default_var_value(&mut self) -> Option<T> {
        self.default_var_value.take()
    }
}
