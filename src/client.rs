mod links {
    fn base_url() -> String {
        "127.0.0.1:8080".to_string()
    }
    pub(crate) fn ready_url() -> String {
        base_url() + r#"/ready"#
    }

    pub(crate) fn exampletranslate() -> String {
        base_url() + r#"/exampletranslate"#
    }

    pub(crate) fn examplenewtrans() -> String {
        base_url() + r#"/examplenewtrans"#
    }

    pub(crate) fn translate() -> String {
        base_url() + r#"/translate"#
    }

    pub(crate) fn newtrans() -> String {
        base_url() + r#"/newtrans"#
    }
}

pub mod maintenance {

    use crate::request::get;

    use super::links::ready_url;

    pub enum Ready {
        Yes,
        No,
    }

    pub async fn ready() -> Ready {
        let result = get(&ready_url(), "".to_owned(), "".to_owned()).await;
        match result {
            Ok(v) => Ready::Yes,
            Err(e) => Ready::No,
            _ => Ready::No,
        }
    }
}

pub mod examples {

    use futures::executor::block_on;

    use super::links;
    use crate::request::get;

    pub fn exampletranslate() -> Result<String, String> {
        let result = block_on(get(
            &links::exampletranslate(),
            "".to_owned(),
            "".to_owned(),
        ));

        match result {
            Ok(v) => {
                let body = block_on(v.text());
                match body {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e.to_string()),
                }
            }

            Err(e) => Err(e.to_string()),
            _ => Err("".to_string()),
        }
    }
}
