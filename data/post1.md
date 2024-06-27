# Notes on Parsing Addresses #

As someone new to writing parsers, I find them anything but comforting.  Entering new domains of computer science feels like wandering into an active minefield occupied by hostile forces.  That feeling comes from the sheer breadth of the subjects that computer programs address, and each subject brings a unique set of problems that seasoned veterans quaintly call domain-specific.  This is industry-speak for feeling like a rookie again.  Something about this field has me feeling like a perennial freshman who is still casting about.

When adventuring forth into a strange new domain, take the best tools.  From the Ecosystem tab of the stats page of [lib.rs](https://lib.rs/stats), 56% of the parser generator crates in the sample used `nom` as a dependency, with `pest` taking second at 28% followed by `combine` at 12%.  The incumbent has a massive advantage going into the selection process, being well established in the field and having thorough documentation, tutorials and associated articles and walkthroughs available, as well as a history of questions and answers to query on internet forums.  I felt that the `pest` and `combine` libraries addressed edge-cases or areas of specific interest to specialists.  If I get really into parsing, these crates might become a better choice, but I need Parsing 101, so I went with `nom`.

## Bringing Order to Chaos

Physical addresses are highly-structured data used to provide emergency services, necessities like water, sewer and electricity, and vital services like sanitation, internet and mail.  Governing bodies over the structure of physical addresses in the US include the FGDC and NENA standards. To give you a sounding of the deep waters into which we are wading, here is an enumeration of the elements in a physical address:

* Address number prefix
* Address number
* Address number suffix
* Street name pre directional
* Street name pre modifier
* Street name pre type
* Street name pre type separator
* Street name
* Street name post type
* Street name post directional
* Street name post modifier
* Incorporated Municipality
* Unincorporated Community
* Postal Community
* State name
* Postal code
* County name
* Country name

The first good test of my address parser came during a survey from the city council.  They were interested in the spatial distribution of responses, and requested that respondents submit their address.  To my delight, this question on the survey form was an open text field.  No data validation was applied.  This dataset is a microcosm of the set of larger problems that I deal with everyday, validating address *blobs*.

An address *blob* refers to a string of text that may or may not contain a valid address, which may or may not be formatted correctly, and may contain honest mistakes or omissions from a well-intended author.
