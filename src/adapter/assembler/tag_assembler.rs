use crate::adapter::api::blog_model::TagVo;
use crate::adapter::repository::po::tag::Model as Tag;

pub fn po_2_vo(tag: Tag) -> TagVo {
    TagVo { name: tag.name }
}
