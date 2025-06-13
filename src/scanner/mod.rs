use anyhow::Result;
use crate::database::DatabaseManager;
use strsim::{jaro_winkler, levenshtein};
use crate::orbital::models::OrbitalAsset;
use crate::orbital::api::OrbitalApiClient;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;