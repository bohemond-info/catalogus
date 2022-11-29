# Build Instructions

## Download Rust, Code
* Go to https://www.rust-lang.org/tools/install
* Find an appropriate directory and get repo: `git clone git@github.com:nathanielford/bohemond-info.git`
* `cd bohemond-info` - essentially, go into the repo directory. Everything else assumes this as a root.

## Export CSV
* Go to `CatalogusExport` sheet. Download this sheet as a csv.
  * Note that the values in this sheet will be the values reflected in the eventual data variable. There is not much in the way of validation yet.
* Rename the csv as `catalogus.csv`. Place in resources folder `api-bohemond/resources/catalogus.csv`.

## Convert to JavaScript file:
Using this quick-and-dirty method you can populate the data variable in the js file as it stands now.
* Go to `cli` sub-directory.
* Run `cargo run`.
* A new data file should appear, `data.js`. This can be copy-pasted straight into your javascript file.

# TODO
* Conversion library.
  * [Done] Basic Conversion 
  * Custom de/serializers to handle validation and cleaner output.
* [Done] Server
* REST API
  * Endpoint: by-list -> List of items
    * Whole list (return the entire dataset)
    * Paginated
  * Endpoint: by-id -> List of items
  * Endpoint: by-location bounded box -> List of items
* API deployment glue
  * Docker build file, supporting scripts.
  * [Done] Set up ip routing/lb
  * [Done] Set up pod for webserver
  * Expose Server via LB