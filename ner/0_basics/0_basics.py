text = """
Sehr geehrte Damen und Herren! Liebe Kolleginnen
und Kollegen! Ich wünsche allen einen schönen guten
Morgen. Die Sitzung ist eröffnet.
Vor Eintritt in die Tagesordnung gratuliere ich nachträglich der Kollegin Heike Brehmer zu ihrem 60. Geburtstag. Im Namen des ganzen Hauses alles Gute!
"""

import spacy
from spacy import displacy

nlp = spacy.load("de_core_news_sm")
doc = nlp(text)

# used pipes
# print(nlp.pipe_names)

displacy.render(doc, style="ent", jupyter=True)
