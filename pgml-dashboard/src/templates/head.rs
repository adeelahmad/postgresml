use sailfish::TemplateOnce;

#[derive(Clone, Default)]
pub struct Head {
    pub title: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub preloads: Vec<String>,
}

impl Head {
    pub fn new() -> Head {
        Head::default()
    }

    pub fn add_preload(&mut self, preload: &str) -> &mut Self {
        self.preloads.push(preload.to_owned());
        self
    }

    pub fn title(mut self, title: &str) -> Head {
        self.title = title.to_owned();
        self
    }

    pub fn description(mut self, description: &str) -> Head {
        self.description = Some(description.to_owned());
        self
    }

    pub fn image(mut self, image: &str) -> Head {
        self.image = Some(image.to_owned());
        self
    }

    pub fn not_found() -> Head {
        Head::new().title("404 - Not Found")
    }
}

#[derive(TemplateOnce, Default, Clone)]
#[template(path = "layout/head.html")]
pub struct DefaultHeadTemplate {
    pub head: Head,
}

impl DefaultHeadTemplate {
    pub fn new(head: Option<Head>) -> DefaultHeadTemplate {
        let head = match head {
            Some(head) => head,
            None => Head::new(),
        };

        DefaultHeadTemplate { head }
    }
}

impl From<DefaultHeadTemplate> for String {
    fn from(layout: DefaultHeadTemplate) -> String {
        layout.render_once().unwrap()
    }
}

#[cfg(test)]
mod head_tests {
    use crate::templates::Head;

    #[test]
    fn new_head() {
        let head = Head::new();
        assert_eq!(
            (head.title, head.description, head.image, head.preloads),
            ("".to_string(), None, None, vec![])
        );
    }

    #[test]
    fn add_preload() {
        let mut head = Head::new();
        let mut preloads: Vec<String> = vec![];
        for i in 0..5 {
            preloads.push(format!("image/test_preload_{}.test", i).to_string());
        }
        for preload in preloads.clone() {
            head.add_preload(&preload);
        }
        assert!(head.preloads.eq(&preloads));
    }

    #[test]
    fn add_title() {
        let head = Head::new().title("test title");
        assert_eq!(head.title, "test title");
    }

    #[test]
    fn add_description() {
        let head = Head::new().description("test description");
        assert_eq!(head.description, Some("test description".to_string()));
    }

    #[test]
    fn add_image() {
        let head = Head::new().image("images/image_file_path.jpg");
        assert_eq!(head.image, Some("images/image_file_path.jpg".to_string()));
    }

    #[test]
    fn not_found() {
        let head = Head::not_found();
        assert_eq!(head.title, "404 - Not Found")
    }
}

#[cfg(test)]
mod default_head_template_test {
    use super::{DefaultHeadTemplate, Head};
    use sailfish::TemplateOnce;

    #[test]
    fn default() {
        let head = DefaultHeadTemplate::new(None);
        let rendered = head.render_once().unwrap();

        assert!(
            rendered.contains(r#"<head>"#) &&
            rendered.contains(r#"<title> – PostgresML</title>"#) &&
            rendered.contains(r#"<meta name="description" content="Train and deploy models to make online predictions using only SQL, with an open source Postgres extension.">"#) &&
            !rendered.contains("preload") &&
            rendered.contains("</head>")
        )
    }

    #[test]
    fn set_head() {
        let mut head_info = Head::new()
            .title("test title")
            .description("test description")
            .image("image/test_image.jpg");
        head_info.add_preload("image/test_preload.webp");

        let head = DefaultHeadTemplate::new(Some(head_info));
        let rendered = head.render_once().unwrap();
        assert!(
            rendered.contains("<title>test title – PostgresML</title>") &&
            rendered.contains(r#"<meta name="description" content="test description">"#) &&
            rendered.contains(r#"<meta property="og:image" content="image/test_image.jpg">"#) && 
            !rendered.contains(r#"<link rel="preload" fetchpriority="high" as="image" href="image/test_preload.webp" type="image/webp">"#)
        );
    }
}
