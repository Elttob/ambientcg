use std::collections::{HashMap, HashSet};

use serde::{Serialize, Deserialize};

use crate::{response::{self, asset::DownloadFolder}, json::util};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    assetId: String,
    releaseDate: String,
    earlyReleaseDate: String,
    dataType: String,
    creationMethod: String,
    category: String,

    // StatisticsData
    downloadCount: Option<u32>,
    downloadCountMonth: Option<u32>,
    downloadCountWeek: Option<u32>,
    popularityScore: Option<f32>,

    // TagData
    tags: Option<HashMap<String, String>>,

    // DisplayData
    dataTypeName: Option<String>,
    dataTypeDescription: Option<String>,
    creationMethodName: Option<String>,
    creationMethodDescription: Option<String>,
    displayName: Option<String>,
    customDisplayName: Option<String>,
    description: Option<String>,
    displayCategory: Option<String>,
    shortLink: Option<String>,

    // DimensionsData
    dimensionX: Option<f32>,
    dimensionY: Option<f32>,
    dimensionZ: Option<f32>,

    // RelationshipData
    createdUsing: Option<Vec<String>>,
    basedOnThis: Option<Vec<String>>,

    // NeighbourData
    nextAssetId: Option<String>,
    previousAssetId: Option<String>,

    // VariationsData
    variations: Option<Vec<String>>,

    // DownloadData
    downloadFolders: Option<HashMap<String, super::DownloadFolder>>,

    // PreviewData
    previewLinks: Option<Vec<super::PreviewLink>>,
    previewType: Option<String>,

    // MapData
    maps: Option<Vec<String>>,

    // UsdData
    hasUsd: Option<bool>,

    // ImageData
    previewImage: Option<HashMap<String, String>>
}

impl From<Asset> for response::asset::Asset {
    fn from(json: Asset) -> Self {
        Self {
            asset_id: json.assetId,
            release_date: util::parse_date(&json.releaseDate),
            early_release_date: util::parse_date(&json.earlyReleaseDate),
            data_type: json.dataType.as_str().parse().unwrap(),
            creation_method: json.creationMethod.as_str().parse().unwrap(),
            category: json.category,
            
            // StatisticsData
            download_count: json.downloadCount,
            download_count_month: json.downloadCountMonth,
            download_count_week: json.downloadCountWeek,
            popularity_score: json.popularityScore,

            // TagData
            tags: match json.tags {
                Some(map) => {
                    let num_tags = map.len();
                    let mut vec = Vec::new();
                    for index in 0..num_tags {
                        let key = (index + 1).to_string();
                        let tag = &map[&key];
                        vec.insert(index, tag.clone());
                    }
                    Some(vec)
                },
                None => None
            },

            // DisplayData
            data_type_name: util::non_empty_str(json.dataTypeName),
            data_type_description: util::non_empty_str(json.dataTypeDescription),
            creation_method_name: util::non_empty_str(json.creationMethodName),
            creation_method_description: util::non_empty_str(json.creationMethodDescription),
            display_name: util::non_empty_str(json.displayName),
            custom_display_name: util::non_empty_str(json.customDisplayName),
            description: util::non_empty_str(json.description),
            display_category: util::non_empty_str(json.displayCategory),
            short_link: util::non_empty_str(json.shortLink),

            // DimensionsData
            dimension_x: util::non_zero_f32(json.dimensionX),
            dimension_y: util::non_zero_f32(json.dimensionY),
            dimension_z: util::non_zero_f32(json.dimensionZ),

            // RelationshipData
            created_using: json.createdUsing,
            based_on_this: json.basedOnThis,

            // NeighbourData
            next_asset_id: json.nextAssetId,
            previous_asset_id: json.previousAssetId,

            // VariationsData
            variations: json.variations,

            // DownloadData
            download_folders: match json.downloadFolders {
                Some(folders_json) => {
                    let mut folders = HashMap::new();

                    for (name, contents_json) in folders_json {
                        let mut categories = HashMap::new();

                        for (title, category_json) in contents_json.downloadFiletypeCategories {
                            let mut downloads = Vec::new();

                            for download in category_json.downloads {
                                downloads.push(response::asset::DownloadFile {
                                    full_download_path: download.fullDownloadPath,
                                    download_link: download.downloadLink,
                                    file_name: download.fileName,
                                    size: download.size,
                                    attribute: util::non_empty_str(download.attribute.into()),
                                    file_type: download.fileType.parse().unwrap(),
                                    zip_content: download.zipContent,
                                })
                            }
                            
                            let category = response::asset::DownloadCategory {
                                file_type: title.parse().unwrap(),
                                downloads: downloads
                            };
                            categories.insert(category.file_type.clone(), category);
                        }

                        let folder = response::asset::DownloadFolder {
                            title: contents_json.title,
                            download_filetype_categories: categories
                        };
                        folders.insert(name, folder);
                    }

                    Some(folders)
                },
                None => None
            },

            // PreviewData
            preview_links: match json.previewLinks {
                Some(vec) => {
                    let new_vec = vec.into_iter()
                        .map(|x| response::asset::PreviewLink::from(x))
                        .collect();
                    Some(new_vec)
                },
                None => None
            },
            preview_type: json.previewType,

            // MapData
            maps: match json.maps {
                Some(str_vec) => {
                    let mut set = HashSet::new();
                    for str in str_vec {
                        set.insert(str.parse().unwrap());
                    }
                    Some(set)
                }
                None => None
            },

            // UsdData
            has_usd: json.hasUsd,

            // ImageData
            preview_images: match json.previewImage {
                Some(str_map) => {
                    let mut img_map = HashMap::new();
                    for (info, url) in str_map {
                        img_map.insert(info.parse().unwrap(), url);
                    }
                    Some(img_map)
                }
                None => None
            }
        }
    }
}