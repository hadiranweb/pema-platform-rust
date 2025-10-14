use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use dtos::page::{CreatePageDto as DtosCreatePageDto, UpdatePageDto as DtosUpdatePageDto};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreatePageDto {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub is_published: bool,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdatePageDto {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub is_published: Option<bool>,
}

impl From<CreatePageDto> for DtosCreatePageDto {
    fn from(dto: CreatePageDto) -> Self {
        Self {
            title: dto.title,
            slug: dto.slug,
            content: dto.content,
        }
    }
}

impl From<UpdatePageDto> for DtosUpdatePageDto {
    fn from(dto: UpdatePageDto) -> Self {
        Self {
            title: dto.title,
            slug: dto.slug,
            content: dto.content,
        }
    }
}

