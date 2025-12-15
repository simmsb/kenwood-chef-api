use color_eyre::eyre::OptionExt as _;
use color_eyre::eyre::eyre;
use sea_orm::DatabaseConnection;
use sea_orm::EntityLoaderTrait;
use sea_orm::EntityLoaderTrait as _;
use sea_orm::QueryFilter;

use crate::entities;
use crate::entities::prelude::*;

pub async fn list_recipes(
    db: &DatabaseConnection,
    offset: Option<u64>,
    limit: Option<u64>,
) -> color_eyre::Result<Vec<types::Recipe>> {
    let mut recipe_models_q = Recipe::load().order_by_id_asc().with(Author);

    if let Some(offset) = offset {
        recipe_models_q.query().offset(offset);
    }

    if let Some(limit) = limit {
        recipe_models_q.query().limit(limit);
    }

    let recipe_models = recipe_models_q.all(db).await?;

    let recipes = recipe_models
        .into_iter()
        .map(|r| {
            Ok(types::Recipe {
                author: {
                    let entities::author::ModelEx {
                        name, image, url, ..
                    } = r.author.into_option().ok_or_eyre("Author not loaded")?;
                    types::Author { name, image, url }
                },
                created_at: r.created_at,
                created_by_id: r.created_by_id,
                description: r.description,
                etag: r.e_tag,
                forked_into_other_locales: Vec::new(),
                id: r.id,
                ingredients: serde_json::from_value(r.ingredients)?,
                locale: r.locale,
                modified_at: r.modified_at,
                name: r.name,
                organization_id: "".to_owned(),
                published_at: r.published_at,
                reference_tags: Vec::new(),
                serves: r.serves as u8,
                state: "published".to_owned(),
                steps: serde_json::from_value(r.steps)?,
                total_time: r
                    .total_time
                    .parse::<jiff::Span>()
                    .map_err(|e| eyre!("Parsing an iso8601_duration: {e:?}"))?
                    .to_duration(jiff::SpanRelativeTo::days_are_24_hours())?,
                visibility: "all-users".to_owned(),
                cook_time: r.cook_time.and_then(|x| {
                    x.parse::<jiff::Span>()
                        .ok()?
                        .to_duration(jiff::SpanRelativeTo::days_are_24_hours())
                        .ok()
                }),
                prep_time: r.prep_time.and_then(|x| {
                    x.parse::<jiff::Span>()
                        .ok()?
                        .to_duration(jiff::SpanRelativeTo::days_are_24_hours())
                        .ok()
                }),
                referenced: None,
                requester_role: None,
            })
        })
        .collect::<color_eyre::Result<Vec<_>>>()?;

    Ok(recipes)
}

pub async fn get_recipe(db: &DatabaseConnection, id: &str) -> color_eyre::Result<types::Recipe> {
    let r = Recipe::load()
        .filter_by_id(id)
        .with(Author)
        .one(db)
        .await?
        .ok_or_eyre("Recipe not found")?;

    Ok(types::Recipe {
        author: {
            let entities::author::ModelEx {
                name, image, url, ..
            } = r.author.into_option().ok_or_eyre("Author not loaded")?;
            types::Author { name, image, url }
        },
        created_at: r.created_at,
        created_by_id: r.created_by_id,
        description: r.description,
        etag: r.e_tag,
        forked_into_other_locales: Vec::new(),
        id: r.id,
        ingredients: serde_json::from_value(r.ingredients)?,
        locale: r.locale,
        modified_at: r.modified_at,
        name: r.name,
        organization_id: "".to_owned(),
        published_at: r.published_at,
        reference_tags: Vec::new(),
        serves: r.serves as u8,
        state: "published".to_owned(),
        steps: serde_json::from_value(r.steps)?,
        total_time: r
            .total_time
            .parse::<jiff::Span>()
            .map_err(|e| eyre!("Parsing an iso8601_duration: {e:?}"))?
            .to_duration(jiff::SpanRelativeTo::days_are_24_hours())?,
        visibility: "all-users".to_owned(),
        cook_time: r.cook_time.and_then(|x| {
            x.parse::<jiff::Span>()
                .ok()?
                .to_duration(jiff::SpanRelativeTo::days_are_24_hours())
                .ok()
        }),
        prep_time: r.prep_time.and_then(|x| {
            x.parse::<jiff::Span>()
                .ok()?
                .to_duration(jiff::SpanRelativeTo::days_are_24_hours())
                .ok()
        }),
        referenced: None,
        requester_role: None,
    })
}
