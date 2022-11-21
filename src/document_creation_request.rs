use crate::Args;

pub struct DocumentCreationRequest {
    pub index: String,
    pub template: String,
    pub count: usize,
}

impl<'a> TryFrom<&'a Args> for Vec<DocumentCreationRequest> {
    type Error = anyhow::Error;

    fn try_from(value: &'a Args) -> Result<Self, Self::Error> {
        let mut output = Vec::<DocumentCreationRequest>::new();

        let indexes = value.index.len();

        if indexes != value.template.len() || indexes != value.count.len() {
            return Err(anyhow::anyhow!(
                "index and count arguments should be present for every template"
            ));
        }

        for i in 0..value.index.len() {
            output.push(DocumentCreationRequest {
                index: value
                    .index
                    .get(i)
                    .ok_or(anyhow::anyhow!("missing index"))?
                    .to_owned(),
                count: value
                    .count
                    .get(i)
                    .ok_or(anyhow::anyhow!("missing count"))?
                    .clone(),
                template: value
                    .template
                    .get(i)
                    .ok_or(anyhow::anyhow!("missing template"))?
                    .to_owned(),
            });
        }

        Ok(output)
    }
}
