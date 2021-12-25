package main

import (
	"fmt"

	textrank "github.com/DavidBelicza/TextRank"
)

func main() {
	rawText := `
In Deutschland leben immer mehr Menschen mit Behinderung (https://www.re
hadat-statistik.de/statistiken/behinderung/schwerbehindertenstatistik/). Der
demografische Wandel wird diesen Trend noch verstärken, denn mit zunehmendem Alter sind immer mehr Menschen in ihrer Teilhabe beeinträchtigt, erkranken chronisch oder es entsteht Pflegebedarf (https://www.pkv.de/wissen/pflege
pflichtversicherung/die-pflege-in-einer-alternden-gesellschaft/). Ende 2019
lebten bereits 7,9 Millionen schwerbehinderte in Deutschland (vgl. Ausführungen im ersten Link).
Die Inklusion von Menschen mit Behinderung ist in Deutschland in Artikel 3
Absatz 3 GG festgelegt und ist ein gesellschaftliches und politisches Ziel.
Deutschland hat sich auch international zum Schutz der Rechte von Menschen
mit Behinderungen verpflichtet durch die Unterzeichnung der UN-Konvention
über die Rechte von Menschen mit Behinderungen (UN-BRK).
Die Corona-Pandemie hat die Inklusion und Betreuung von Menschen mit Behinderung jedoch zusätzlich vor schwierige Aufgaben gestellt (https://www.vd
k.de/deutschland/pages/themen/behinderung/behindertenpolitik/79418/inklusio
n_protesttag?dscc=ok).
So sind durch Corona die Arbeitslosenzahlen bei Menschen mit Behinderung
erheblich angestiegen (https://www.tagesschau.de/wirtschaft/konjunktur/arbeits
markt-behinderung-inklusion-101.html).
Auch in Nicht-Krisenzeiten geben wenige Arbeitgeber Menschen mit Behinderungen eine Chance. Der Übergang aus Werkstätten für Menschen mit Behinderungen (WfbM) in den allgemeinen Arbeitsmarkt stellt sich für die Arbeitnehmer oftmals als unüberwindbares Hindernis dar. Viele Betroffene empfinden
Behindertenwerkstätten als Widerspruch zu Artikel 27 UN-BRK, wonach behinderte Menschen gleiches Recht auf Arbeit und Verdienst ihres Lebensunterhalts haben. Die leider oftmals alternativlose Arbeit in WfbM ist häufig nicht
frei gewählt und die Beschäftigten können ihren Lebensunterhalt damit nicht
bestreiten (https://jobinklusive.org/2020/09/14/wie-das-system-der-behinderten
werkstaetten-inklusion-verhindert-und-niemand-etwas-daran-aendert/).
Fatal ist nach Auffassung der Fragesteller die Situation, die durch die Maßnahmen der Regierung zur Eindämmung der Corona-Pandemie vor allem in Betreuungseinrichtungen entstanden ist, da diese zeitweise ganz geschlossen sind
oder oft nur eine Notbetreuung anbieten`

	// TextRank object
	tr := textrank.NewTextRank()
	// Default Rule for parsing.
	rule := textrank.NewDefaultRule()
	// Default Language for filtering stop words.
	language := textrank.NewDefaultLanguage()
	// Default algorithm for ranking text.
	algorithmDef := textrank.NewDefaultAlgorithm()

	// Add text.
	tr.Populate(rawText, language, rule)
	// Run the ranking.
	tr.Ranking(algorithmDef)

	sentences := textrank.FindSentencesByRelationWeight(tr, 10)
	// Found sentences
	fmt.Println(sentences)
}
