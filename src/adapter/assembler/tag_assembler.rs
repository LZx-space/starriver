use crate::adapter::api::blog_model::TagVo;
use crate::domain::tag::aggregate::Tag;

pub fn po_2_vo(tag: Tag) -> TagVo {
    TagVo { name: tag.name }
}
