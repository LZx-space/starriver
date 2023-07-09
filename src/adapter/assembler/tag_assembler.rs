use crate::adapter::api::blog_model::TagVo;
use crate::adapter::repository::tag_po::Model as Tag;

pub fn po_2_vo(tag: Tag) -> TagVo {
    TagVo { name: tag.name }
}
