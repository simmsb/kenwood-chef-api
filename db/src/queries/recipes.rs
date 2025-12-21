use color_eyre::eyre::OptionExt as _;
use color_eyre::eyre::WrapErr as _;
use color_eyre::eyre::eyre;
use sea_orm::EntityLoaderTrait as _;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelBehavior as _, EntityTrait as _};
use sea_orm::{ActiveValue::Set, DatabaseConnection};
use sea_orm::{
    ActiveValue::NotSet,
    ColumnTrait as _, EntityLoaderTrait,
};

use crate::entities::{self, author, prelude::*, recipe};

pub async fn list_recipes(
    db: &DatabaseConnection,
    offset: Option<u64>,
    limit: Option<u64>,
    all: bool,
) -> color_eyre::Result<Vec<types::Recipe>> {
    let mut recipe_models_q = Recipe::load().order_by_id_asc().with(Author);

    if let Some(offset) = offset {
        recipe_models_q.query().offset(offset);
    }

    if let Some(limit) = limit {
        recipe_models_q.query().limit(limit);
    }

    if !all {
        recipe_models_q.filter_mut(recipe::Column::IsCustom.eq(true))
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
                // ingredients: serde_json::from_value(r.ingredients)?,
                ingredients: serde_path_to_error::deserialize(r.ingredients)
                    .with_context(|| format!("Deserializing ingredients of {}", r.id))?,
                // steps: serde_json::from_value(r.steps)?,
                steps: serde_path_to_error::deserialize(r.steps)
                    .with_context(|| format!("Deserializing steps of {}", r.id))?,
                id: r.id,
                locale: r.locale,
                modified_at: r.modified_at,
                name: r.name,
                organization_id: "".to_owned(),
                published_at: r.published_at,
                reference_tags: Vec::new(),
                serves: r.serves as u8,
                state: "published".to_owned(),
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

pub async fn list_recipe_items(
    db: &DatabaseConnection,
    offset: Option<u64>,
    limit: Option<u64>,
    all: bool,
) -> color_eyre::Result<Vec<types::RecipeItem>> {
    let mut recipe_models_q = Recipe::load().order_by_id_asc().with(Author);

    if let Some(offset) = offset {
        recipe_models_q.query().offset(offset);
    }

    if let Some(limit) = limit {
        recipe_models_q.query().limit(limit);
    }

    if !all {
        recipe_models_q.filter_mut(recipe::Column::IsCustom.eq(true))
    }

    let recipe_models = recipe_models_q.all(db).await?;

    let recipes = recipe_models
        .into_iter()
        .map(|r| {
            Ok(types::RecipeItem {
                id: r.id,
                name: r.name,
                author_name: r.author.into_option().ok_or_eyre("Author not loaded")?.name,
                total_time: r
                    .total_time
                    .parse::<jiff::Span>()
                    .map_err(|e| eyre!("Parsing an iso8601_duration: {e:?}"))?
                    .to_duration(jiff::SpanRelativeTo::days_are_24_hours())?,
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
        // ingredients: serde_json::from_value(r.ingredients)?,
        ingredients: serde_path_to_error::deserialize(r.ingredients)
            .context("Deserializing ingredients")?,
        locale: r.locale,
        modified_at: r.modified_at,
        name: r.name,
        organization_id: "".to_owned(),
        published_at: r.published_at,
        reference_tags: Vec::new(),
        serves: r.serves as u8,
        state: "published".to_owned(),
        // steps: serde_json::from_value(r.steps)?,
        steps: serde_path_to_error::deserialize(r.steps).context("Deserializing steps")?,
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

// bad api, but IDC
// TODO: Port Rel8 to Rust
pub async fn set_recipe(
    db: &DatabaseConnection,
    r: types::Recipe,
    create: bool,
) -> color_eyre::Result<()> {
    let author = Author::find()
        .filter(author::Column::Name.eq(&r.author.name))
        .one(db)
        .await?;

    let author_id = match author {
        Some(author) => author.id,
        None => {
            entities::author::ActiveModelEx {
                id: NotSet,
                name: Set(r.author.name),
                image: Set(r.author.image),
                url: Set(r.author.url),
                recipes: sea_orm::HasManyModel::NotSet,
            }
            .insert(db)
            .await?
            .id
        }
    };

    let model = entities::recipe::ActiveModelEx {
        author: sea_orm::HasOneModel::NotSet,
        author_id: Set(author_id),
        id: Set(r.id.clone()),
        name: Set(r.name.clone()),
        description: Set(r.description.clone()),
        prep_time: Set(r.prep_time.map(|x| x.to_string())),
        cook_time: Set(r.cook_time.map(|x| x.to_string())),
        total_time: Set(r.total_time.to_string()),
        serves: Set(r.serves as i64),
        e_tag: Set(r.etag.clone()),
        organisation_id: Set(r.organization_id.clone()),
        locale: Set(r.locale.clone()),
        created_at: Set(r.created_at),
        modified_at: Set(r.modified_at),
        published_at: Set(r.published_at),
        created_by_id: Set(r.created_by_id.clone()),
        steps: Set(serde_json::to_value(&r.steps).unwrap()),
        ingredients: Set(serde_json::to_value(&r.ingredients).unwrap()),
        is_custom: Set(true),
    };

    if create {
        model.insert(db).await?;
    } else {
        model.update(db).await?;
    }

    Ok(())
}
