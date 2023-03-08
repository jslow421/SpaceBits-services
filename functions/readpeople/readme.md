# Read People In Space

Function to read json data from S3 bucket and return people currently in space

## Build for prod arm64 release (graviton)
`cargo lambda build --arm64 --release`
`cargo lambda build --arm64 --release -l ../out/`

## Serve
`cargo lambda watch`

## Test after serving
`cargo lambda invoke --data-ascii "{ \"command\": \"hi\" }"`
