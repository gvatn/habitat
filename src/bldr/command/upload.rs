//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Uploads a package to a [repository](../repo).
//!
//! # Examples
//!
//! ```bash
//! $ bldr upload chef/redis -u http://localhost:9632
//! ```
//!
//! Will upload a package to the repository.
//!
//! # Notes
//!
//! This should be extended to cover uploading specific packages, and finding them by ways more
//! complex than just latest version.
//!

use error::BldrResult;
use config::Config;

use pkg::Package;
use repo;

/// Upload a package to a repository.
///
/// Find the latest package, then read it from the cache, and upload to the repository.
///
/// # Failures
///
/// * Fails if it cannot find a package
/// * Fails if the package doesn't have a `.bldr` file in the cache
/// * Fails if it cannot upload the file
pub fn package(config: &Config) -> BldrResult<()> {
    let url = config.url().as_ref().unwrap();
    let package = try!(Package::latest(config.deriv(), config.package(), None, None));
    println!("   {}: Uploading from {}",
             &package,
             package.cache_file().to_string_lossy());
    try!(repo::client::put_package(url, &package));
    println!("   {}: complete", config.package());
    Ok(())
}
