use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::{validate_email, Validate, ValidationError};
use regex::Regex;

use crate::models::{Duration, User};