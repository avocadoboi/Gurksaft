# To do
* Test multiple words in a single sentence. One word is chosen as before based on weight, and all words in that sentence that have a weight higher than that word will also be tested.
* Add statistics/progress page with scores such as "expected" word index from the word weight distribution, the word index with the biggest weight, plots of these over time, a histogram that can be zoomed and scrolled horizontally over weights ordered by word frequency, etc. 
* Improve error handling.
* Maybe find out a way to count the number of words the user has "learned" (gotten correct enough times? in a row?) and add that as a metric on the statistics page.
* Add a list of the 10 or something words with the biggest weights, either showing them on the statistics page, between tasks with a certain interval that can be set in the options, or both. Showing them between tasks with a certain interval could help give an overview of what words are most likely to come up. Maybe add some more information about the words in the list, a short example sentence or something.
* Maybe fetch definitions for words and incorporate them in some way.
* Add better support for right-to-left languages.
