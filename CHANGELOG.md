# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.9.0] 2021-04-09
### Added
* geo-lat-long

### Changed
* Enum based errors for check results
* Restructured check result to make url always available

## [0.8.4] 2021-04-09
### Added
* Return if there was an ssl error on each check

### Changed
* Do not fail on SSL errors but retry on error with ignore ssl errors
* Decode charset of content type if existing

## [0.8.3] 2021-04-05
### Changed
* Always ignore content-type "text/html"

## [0.8.2] 2021-04-03
### Changed
* Early return for results that are surely streams

## [0.8.1] 2021-02-18
### Changed
* Use header "icy-samplerate" as fallback for "icy-sr"

### Fixed
* Support multiple occurences of "icy-br"
