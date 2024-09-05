# To do and ideas
* Add scores together with graphs over time such as mean word index from the word weight distribution. 
* Improve error handling. We should maybe implement a popup window for some error messages, for example if you are trying to download language data but you are not connected to the internet then we could show an error message in a new window, and after clicking 'retry' or 'cancel' we either immediately try again or go back to the previous screen. Maybe we should have the error message in the same window but on a dedicated page.
* Count a word as 'learned' if the long term memory of the word is above a certain threshold that you can set in the options, and add it together with a graph of it over time to the statistics screen.
* Color the bars on the word memory graph depending on if the word is normal, easy or learned and only show words that have been tested at least once (make long_term_memory Optional?).
* Add a bias towards shorter sentences that weakens as the number of learned words increases.
* Add a slow listening button and a slowness parameter for this button in the options.
* Add an option for whether the audio should play automatically after each guess.
* Internationalize user interface.
* Maybe fetch definitions for words and incorporate them in some way. Use wiktionary and show definitions in the chosen translation languages when clicking on words?
* Add better support for right-to-left languages.
* Make custom toggle buttons for use on the options screen.
* Replace more constants in the code with options.
* Add a loading screen while the save data is being loaded at startup.
* Don't play the same audio twice in a row unless the user explicitly clicked the listen button.