# RANDEVU
Global Probabilistic Daily Reminders for Anything

https://github.com/TypicalHog/randevu

- GLOBAL - reminders for the same object are the same for everyone
- PROBABILISTIC - reminders are calculated using the probabilistic algorithm
- DAILY - reminders are generated on a daily basis (UTC)
- GENERIC - applicable to anything and everything, literally
- FOSS - in public domain (Unlicense or MIT or Apache-2.0)
- OFFLINE - no internet connection required
- DETERMINISTIC - easily computable and predictible for any object and any date
- PSEUDORANDOM - reminders are spaced randomly and uniformly
- ADJUSTABLE - user can decide how frequently they would like to be reminded of each object

---

## KEY CONCEPTS
- OBJECT - a string representing anything that can be represented with a string of characters (game, movie, person, song, place, video, topic, word, book, number, brand, post, event, item, website, app, quote, action or literally anything else)
- DATE - a string representing a date in ISO 8601 format (YYYY-MM-DD)
- RDV - a positive integer representing the level/significance of a reminder for an OBJECT on a specific DATE

## RDV CALCULATION
`RDV = number of leading zero bits in blake3(blake3(OBJECT) || blake3(DATE))`

## HOW IT WORKS AND POTENTIAL USECASES
Imagine a system which assigns a special number (RDV) to every object each day.  
The number assigned to each object is different for each object and changes daily.  
The number has a 50% chance to be 0, 25% chance to be 1, 12.5% to be 2 and so on (each one being twice as rare).  
If an object has RDV4, that also implies RDV3, RDV2, RDV1 and RDV0.  
User can then choose to set a threshold value for each object and if the RDV value for a specific object is greather than or equal the threshold - user may decide to do something with that object.  
For example, one may decide to watch a certain video once its RDV value hits their desired threshold.  
Threshold allows one to decide how often they would like to be "reminded of" a certain object.  
0 -> every day, 1 -> every 2 days on average, 2 -> 4 days, 3 -> 8 days and so on (allows for essentially infinite frequencies of reminders, though ones above 10 happen quite rarely - 2^10 = 1024 days).  
If multiple people used this system to get reminded of the same things - they would all get the reminded of them on the same days and thus be able to coordinate meetings related to the object in question.  
This could allow fans of a "dead" game to all meet and play it on the same day, like once every 256 days let's say.  
People could re-watch their favorite movie or video and discuss them with the fellow fans on the same day.  
This system can be aplied to anything.  
It can be used to assign a special appreciation/remembrance days to your favorite books, songs, artists, events or (as I already said) - ANYTHING.  
One could have a huge list of objects they care about and never again risk forgeting any of them - since they will be reminded of them eventually.

## OBJECT NAMING CONVENTION (non-exhaustive examples)
OBJECT is a string (preferably uppercase A-Z, 0-9). No spaces allowed.  
Spaces and any other characters should be replaced with a single underscore.  
Characters outside of this set should only be used for external identifiers which are case sensitive or contain other symbols, for example YOUTUBE video IDs.

```
XONOTIC (all letters should be uppercase)

STAR_WARS (spaces and dashes in multi-word objects should be replaced with _)

THE_MATRIX_1999 (movies should have a year of release at the end)

GRAND_THEFT_AUTO_5 (objects should be referenced by their full name, roman numerals should be replaced with arabic ones)

ASAP_ROCKY ($ should be replaced with S, same for other similar instances)

C_PLUS_PLUS (++ should be replaced with _PLUS_PLUS)

C_SHARP (# should be replaced with _SHARP, in other contexts it could be _HASH or omitted)

YEAR_2000 (years should have a YEAR_ prefix)

FRIDAY (weekdays and months should not be abbreviated, same as all other objects)

2023-08-25 (dates should be in ISO 8601 format, YYYY-MM-DD)

NO_MANS_SKY (apostrophe in MAN'S should be dropped)

HARRY_POTTER_SMOKES_WEED_Cdfkq2Nmb3c (video ID should be appended to the video title, double underscore is fine if the ID starts with one, IDs are allowed to use dashes and lowecase letters)

GETTING_BANGED_BY_GREEN_BOOMERS_MINECRAFT_BETA_1_7_3_SOLO_SURVIVAL_NO_COMMENTARY_OJzsmWBQE3I (brackets, quotation marks, colons and other punctuation should be dropped, periods in version numbers like 1.7.3 should be replaced with _)

NUMBER_69 (numbers should have a NUMBER_ prefix)
```

### WHY THIS VERY SPECIFIC AND STRICT CONVENTION THO?

**TO MAKE SURE EVERYONE GETS THE SAME REMINDERS FOR THE SAME THINGS ON THE SAME DAYS.**

Due to how the algorithm works - even just a single character difference between two objects causes the system to generate completely different reminders for each.  
For example: WEED, weed, Weed, WEED1, WEED_ and WEED420 would all be treated as different and independent objects with completely unrelated reminders.  
One can think of objects like passwords - same password gets you the same reminders as everyone else.  
If one wanted to get completely different reminders from other people for a specific object - they could append extra characters to it.  
However, this is not the focus of the system, the whole point is to allow people to coordinate getting reminded about stuff at the same time as others.
