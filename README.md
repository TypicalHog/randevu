# randevu
Universal Probabilistic Daily Reminders for Anything

https://github.com/TypicalHog/randevu

- OFFLINE - no internet connection required
- FOSS - unlicensed, in public domain (Unlicense OR MIT OR Apache-2.0)
- DAILY - reminders are generated on a daily basis (UTC)
- DETERMINISTIC - easily reproducible for any object and any date
- PSEUDORANDOM - reminders are spaced randomly and uniformly
- PROBABILISTIC - reminders are calculated using the probabilistic algorithm
- UNIVERSAL - reminders for the same object are the same for everyone
- GENERIC - applicable to anything and everything, literally
- ADJUSTABLE - frequency of reminders for each object can be adjusted

Imagine a system which assigns a special number to every object each day.  
Object can be anything that can be represented with a string of characters.  
That can be a game, movie, person, place, video, topic, word, book, brand, post or literally anything else.  
The number assigned to each object is different for each object and changes each day.  
The number is called RDV and it has a 50% chance to be a 0, 25% chance to be 1, 12.5% to be 2 and so on (each one being twice as rare).  
If an object has RDV4 for example, that also implies RDV3, RDV2, RDV1 and RDV0. 
User can then choose to set a threshold value for each object and if the RDV value for a specific object is greather than or equal to the threshold, 
user may decide to do something that that object.
For example, one may decide to re-watch a certain video once its RDV value hits the threshold.
Threshold allows one to decide how often they would like to be "reminded of" a certain object.
0 -> every day, 1 -> about every 2 days, 2 -> 4 days and so on (allows for essentially infinite frequencies of reminders, though ones above 10 are very rare 2^10 = 1024 days).
If multiple people used this system to get reminded of the same items - they would all get the reminders on the same days and thus be able to coordinate meetings about stuff
like games with low player counts or watching and discussing their favorite obscure movie like once every 2 years with all other fans of that movie on the same day.

The RDV number is calculated according to the following formula:  

>RDV = number of leading zero bits in blake3(blake3(OBJECT) || blake3(DATE))

OBJECT is a string (preferably uppercase A-Z, 0-9). No spaces allowed. 
Spaces and any other characters should be replaced with a single underscore "_". 
Characters outside of this set should only be used for case sensitive external identifiers, 
or ones that contain other symbols and characters, like YOUTUBE links. 

DATE is a string with the date (ISO 8601 format, YYYY-MM-DD). 

OBJECT NAMING CONVENTION EXAMPLES (non-exhaustive):

>**XONOTIC** (all letters should be uppercase)
>
>**THE_MATRIX_1999** (movies should have a year of release at the end)
>
>**GRAND_THEFT_AUTO_5** (objects should be referenced by their full name, roman numerals should be replaced with arabic ones)
>
>**ASAP_ROCKY** ($ should be replaced with S, same for other similar instances)
>
>**NOTEPAD_PLUS_PLUS** (++ should be replaced with _PLUS_PLUS, same for C++)
>
>**C_SHARP** (# should be replaced with _SHARP, in other contexts it could be _HASH or dropped)
>
>**YEAR_2000** (years should have a YEAR_ prefix)
>
>**2023-08-25** (dates should be in ISO 8601 format, YYYY-MM-DD)
>
>**NO_MANS_SKY** (apostrophe in MAN'S should be dropped)
>
>**GETTING_BANGED_BY_GREEN_BOOMERS_MINECRAFT_BETA_1_7_3_SOLO_SURVIVAL_NO_COMMENTARY_OJzsmWBQE3I** (brackets, quotation marks, colons and other punctuation should be dropped, periods in version numbers like 1.7.3 should be replaced with underscores)
>
>**HARRY_POTTER_SMOKES_WEED_Cdfkq2Nmb3c** (video ID should be appended to the video title, double underscore is fine if the ID starts with one)
>
>**NUMBER_420** (numbers should have a NUMBER_ prefix)

WHY THO? YOU MAY ASK.

**TO MAKE SURE EVERYONE GETS THE SAME REMINDERS FOR THE SAME THINGS.**

Even just a single character that's different causes the system to generate completely different reminders. 
You could append certain items with some extra characters to get completely different and unrelated reminders from other people, if you really wanted that. 
WEED, Weed, WEED1, WEED_ and WEED420 are all treated as different objects. 
Think of the object IDs like passwords. Same password gets you the same reminders as other people. 
NOTE: Naming convention may change and evolve in the future in case there are good reasons for that. 
