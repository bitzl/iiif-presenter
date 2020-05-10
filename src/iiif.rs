use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Value {
    Single(String),
    Many(Vec<String>),
    Multilang(Vec<LocalizedValue>),
}

#[derive(Debug, Serialize)]
pub struct LocalizedValue {
    value: String,
    language: String,
}

#[derive(Debug, Serialize)]
pub struct Metadata {
    label: String,
    value: Value,
}

impl Metadata {
    pub fn new(label: &str, value: Value) -> Metadata {
        Metadata {
            label: label.to_owned(),
            value: value,
        }
    }

    pub fn key_value(label: &str, value: &str) -> Metadata {
        Metadata {
            label: label.to_owned(),
            value: Value::Single(value.to_owned()),
        }
    }
}

#[derive(Debug, Serialize)]
// #[serde(serialize_with="serde::with_skip_serializing_none")]
pub struct Manifest {
    #[serde(rename = "@context")]
    context: Uri,
    #[serde(rename = "@id")]
    id: Uri,
    #[serde(rename = "@type")]
    iiif_type: String,
    label: String,
    metadata: Vec<Metadata>,
    description: Option<String>,
    // thumbnail: Image,

    // see_also: Vec<Uri>,
    sequences: Vec<Sequence>,
}

impl Manifest {
    pub fn new(
        base_urls: &BaseUrls,
        item_id: &str,
        label: &str,
        metadata: Vec<Metadata>,
        description: Option<String>,
        // thumbnail: Image,
        // see_also: Repeated<Uri>,
    ) -> Manifest {
        let id = Uri::new(format!("{}/{}/manifest", base_urls.presentation, item_id));
        let context = Uri::new("http://iiif.io/api/presentation/2/context.json".to_owned());
        let sequences: Vec<Sequence> = Vec::new();
        Manifest {
            context: context,
            id: id,
            iiif_type: "sc:Manifest".to_owned(),
            label: label.to_owned(),
            metadata: metadata,
            description: description,
            // thumbnail: thumbnail,
            // see_also: see_also,
            sequences: sequences,
        }
    }

    pub fn add_sequence(&mut self, sequence: Sequence) {
        self.sequences.push(sequence);
    }
}

#[derive(Debug, Serialize)]
pub struct Image {
    id: Uri,
    service: Service,
}

#[derive(Debug, Serialize)]
pub struct Service {
    context: Uri,
    id: Uri,
    profile: Uri,
    protocol: Uri
}

impl Service {
    fn new_image_service(base_urls: &BaseUrls, image_id: &str) -> Service {
        let context = Uri::new("http://iiif.io/api/image/2/context.json".to_owned());
        let id = Uri::new(format!("{}/{}", base_urls.image, image_id));
        let profile = Uri::new("http://iiif.io/api/image/2/level2.json".to_owned());
        let protocol = Uri::new("http://iiiif.io/api/image".to_owned());
        Service{context, id, profile, protocol}
    }
}


#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct Uri {
    value: String,
}

impl Uri {
    pub fn new(value: String) -> Uri {
        Uri {
            value: value.to_owned(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Sequence {
    context: Uri,
    id: Uri,
    #[serde(rename = "type")]
    iiif_type: String,
    label: String,
    canvases: Vec<Canvas>,
}

impl Sequence {
    pub fn new(base_urls: &BaseUrls, item_id: &str, label: &str) -> Sequence {
        let id = Uri::new(format!("{}/{}/sequence/normal", base_urls.presentation, item_id));
        let context = Uri::new("http://iiif.io/api/presentation/2/context.json".to_owned());
        let iiif_type = "sc:Sequence".to_owned();
        let canvases: Vec<Canvas> = Vec::new();
        let label = label.to_owned();
        Sequence {
            context,
            id,
            iiif_type,
            label,
            canvases,
        }
    }

    pub fn add(&mut self, canvas: Canvas) {
        self.canvases.push(canvas);
    }

    pub fn add_image(&mut self, base_urls: &BaseUrls, item_id: &str, image_id: &str, label: &str, image_format: &ImageFormat, width: u64, height: u64) {
        let index = self.canvases.len();
        let mut canvas = Canvas::new(base_urls, item_id, index, label, width, height);
        let image_resource = ImageResource::new(base_urls, item_id, image_id, image_format, width, height);
        let annotation = Annotation::new(base_urls, Resource::Image(image_resource), (&canvas.id).clone());
        &canvas.add_image(annotation);
        self.canvases.push(canvas);
    }
}

pub struct BaseUrls {
    presentation: String,
    image: String
}

impl BaseUrls {
    pub fn new(presentation: String, image: String) -> BaseUrls {
        BaseUrls{presentation, image}
    }
}

#[derive(Debug, Serialize)]
pub struct Canvas {
    id: Uri,
    context: Uri,
    #[serde(rename = "type")]
    iiif_type: String,
    label: String,
    height: u64,
    width: u64,
    images: Vec<Annotation>,
}

impl Canvas {
    pub fn new(base_urls: &BaseUrls, item_id: &str, index: usize, label: &str, width: u64, height:u64) -> Canvas {
        let id = Uri::new(format!("{}/{}/canvas/{}", base_urls.presentation, item_id, index));
        let context = Uri::new("http://iiif.io/api/presentation/2/context.json".to_owned());
        let iiif_type = "sc:Canvas".to_owned();
        let images: Vec<Annotation> = Vec::new();
        let label = label.to_owned();
        Canvas{id, context, iiif_type, label, height, width, images}
    }

    pub fn add_image(&mut self, image: Annotation) {
        self.images.push(image);
    }
}


#[derive(Debug, Serialize)]
pub struct Thumbnail {
    id: Uri,
    iiif_type: String,
    height: u64,
    width: u64,
}

#[derive(Debug, Serialize)]
pub struct Annotation {
    context: Uri,
    #[serde(rename = "type")]
    iiif_type: String,
    motivation: String,
    resource: Resource,
    on: Uri,
}

impl Annotation {
    pub fn new(base_urls: &BaseUrls, resource: Resource, on: Uri) -> Annotation {
        let context = Uri::new("http://iiif.io/api/presentation/2/context.json".to_owned());
        let iiif_type = "oa:Annotation".to_owned();
        let motivation = "sc:painting".to_owned();
        Annotation{context, iiif_type, motivation, resource, on}
    }
}

#[derive(Debug, Serialize)]
pub enum Resource {
    Image(ImageResource),
}

#[derive(Debug, Serialize)]
pub struct ImageResource {
    id: Uri,
    #[serde(rename = "type")]
    iiif_type: String,
    format: String,
    service: Service,
    width: u64,
    height: u64,
}

impl ImageResource {
    pub fn new(base_urls: &BaseUrls, item_id: &str, image_id: &str, image_format: &ImageFormat, width: u64, height:u64) -> ImageResource {
        let id = Uri::new(format!("{}/{}/{}/full/full/default.{}", base_urls.image, item_id, image_id, image_format.extension()));
        let iiif_type = "dctypes:Image".to_owned();
        let service = Service::new_image_service(base_urls, item_id);
        let format = image_format.format().to_owned();
        ImageResource{id, iiif_type, format, service, width, height}
    }
}


pub enum ImageFormat {
    JPEG,
    PNG,
    Unknown
}

impl ImageFormat {
    pub fn extension(&self) -> &str {
        match self {
            &ImageFormat::JPEG => "jpg",
            &ImageFormat::PNG => "png",
            &ImageFormat::Unknown => ""
        }
    }

    pub fn format(&self) -> &str {
        match self {
            &ImageFormat::JPEG => "image/jpeg",
            &ImageFormat::PNG => "image/png",
            &ImageFormat::Unknown => "image/unknown",
        }
    } 
}