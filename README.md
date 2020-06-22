# av-stream-info-rust

Check a http/https address if it leads to an audio or a video stream.
Analyze the stream's metainformation.

It only uses the HTTP header fields and the first 50 bytes to analyze the stream.

## Recognized headers

* **icy-pub** - [Number] Possible values are 0 and 1. 0 means NOT public. 1 means public. (VERSION: 1)
* **icy-audio-info** - [String]  (VERSION: 1)
* **content-type** - [String] Stream encoding type (e.g.: audio/flac)
* **icy-name** - [String] Name of the stream or the station. (e.g.: Smurf City, 88.5) (VERSION: 1)
* **icy-description** - [String] A longer description of a station. (e.g.: The number 1 stream of smurf city!) (VERSION: 1)
* **icy-url** - [String] Homepage of the stream. This is not the stream url, but some kind of station homepage! (e.g.: http://example.com) (VERSION: 1)
* **icy-br** - [Number] Bitrate as a number. (e.g.: 128) (VERSION: 1)
* **icy-genre** - [String] Multiple tags split up by comma that describe the station. (e.g.: jazz,classical) (VERSION: 1)
* **icy-sr** - [Number] Sampling rate of the stream in Hz. (e.g.: 44100) (VERSION: 1)
* **icy-logo** - [String] Url of a logo for this stream, should be in JPG or PNG format. (e.g.: http://example.com/logo.png) (VERSION: 2)
* **icy-main-stream-url** - [String] Link to load balanced version of this stream. This may be used by stream providers to direct indexers to the main publicly exposed url. Indexers should update their database accordingly.(VERSION: 2)
* **icy-version** - [Number] The version of this header. 1 is the default. 2 is an extension to the default which is compatible to 1 but adds more headers) (VERSION: 2)
* **icy-index-metadata** - [Number] Use all of header metadata. This is mainly used to force indexers to update their information. 0 means NO. 1 means YES. (VERSION: 2)
* **icy-country-code** - [String] 2 letter countrycode. (https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) (VERSION: 2)
* **icy-country-subdivision-code** - [String] Code of the subdivision of a country. (https://en.wikipedia.org/wiki/ISO_3166-2) (VERSION: 2)
* **icy-language-codes** - [String] Multiple comma delimited language-codes in the format ISO 639-1 (https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) or ISO 639-3 (https://en.wikipedia.org/wiki/ISO_639-3). (VERSION: 2)
* **icy-do-not-index** - [Number] If a stream operator wants this stream to be absolutely private, this option can be set to 1.

## Additional information

* https://www.stream-meta.info