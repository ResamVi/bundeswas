<template>
  <div class="px-3 flex flex-col items-stretch">
    <div class="border-2 border-gray-500 rounded-lg p-8 my-3">
      <div class="mb-5">
        <p class="mt-4 text-2xl text-gray-900 font-extrabold tracking-tight ">{{ protokoll.titel }}</p>

        <p class="text-sm">
          {{ protokoll.datum }} | <a :href="protokoll.fundstelle.pdf_url" class="text-blue-500 underline"> <!-- TODO: hover over link -->
          {{ protokoll.dokumentnummer }}</a> <font-awesome-icon icon="file-pdf" class="text-blue-500"/>
        </p>
      </div>

      <div v-for="(vorgangsposition, index) in protokoll.vorgangspositionen" :key="index">

        <!-- TODO: Verlinkung zu: https://www.bundestag.de/services/glossar/glossar/D/dritte_lesung-245384 -->
        <h3 class="mt-8 font-semibold text-[#522b20]">
          {{ vorgangsposition.vorgangstyp }} (<span class="italic">{{ vorgangsposition.vorgangsposition }}</span>)
        </h3>

        <p>{{ vorgangsposition.titel }}</p>

        <!-- TODO: Bei Geschäftsordnung klappts nicht -->
        Initiative von <img v-for="(partei, index) in vorgangsposition.vorgang.initiative" :key="index" :src="getFilename(partei)" :alt="partei" width="32" height="32" class="inline m-2"> <br />

        <!-- Manche Vorgänge bedarfen keinen Beschluss (z.B. Rüge, Aktuelle Stunde) -->
        <span :class="colorText(vorgangsposition.vorgang.beratungsstand)">{{ vorgangsposition.vorgang.beratungsstand }} </span>
        <!--<p v-if="vorgangsposition.beschlussfassung && vorgangsposition.beschlussfassung.length == 1" :class="colorText(vorgangsposition.beschlussfassung[0].beschlusstenor)">
          {{ vorgangsposition.beschlussfassung[0].beschlusstenor }}
        </p>-->

        <!-- Bei Überweisung gebe an wohin -->
        <span v-if="vorgangsposition.ueberweisung && vorgangsposition.ueberweisung.length == 1">
          zu {{ vorgangsposition.ueberweisung[0].ausschuss }}
        </span>

        <!-- Nenne nur einen, falls Überweisungen zu mehreren Ausschüssen -->
        <span v-if="vorgangsposition.ueberweisung && vorgangsposition.ueberweisung.length > 1">
          zu {{ vorgangsposition.ueberweisung[0].ausschuss }} und weitere
        </span>


        <br/>
      </div>

    </div>
  </div>
</template>

<script>
export default {
  name: 'Plenarprotokoll',
  props: {
    protokoll: Object
  },
  methods: {
    colorText(beschluss) {
      switch(beschluss) {
        case "Angenommen":
        case "Abgeschlossen":
        case "Überwiesen":
        case "Bundesrat hat zugestimmt":
        case "Annahme":
        case "Annahme Geschäftsordnungsantrag":
        case "Annahme der Wahlvorschläge":
        case "Überweisung":
        case "Annahme der Vorlage": // Was ist damit gemeint?
        case "Erklärung der Vorlage für erledigt": // Was ist damit gemeint?
        case "Annahme in Ausschussfassung": // Was ist damit gemeint?
          return "text-green-600"

        case "Ablehnung der Vorlage":
        case "Abgelehnt":
          return "text-red-600"
      }

      return "text-black-600"
    },

    getFilename(fullname) {
      switch(fullname) {
        case "Bundesregierung":
          return require("./../assets/bundesregierung.svg")
        case "Fraktion BÜNDNIS 90/DIE GRÜNEN":
          return require("./../assets/gruen.svg")
        case "Fraktion der FDP":
          return require("./../assets/fdp.svg")
        case "Fraktion der SPD":
          return require("./../assets/spd.svg")
        case "Fraktion der AfD":
          return require("./../assets/afd.svg")
        case "Fraktion der SSW":
          return require("./../assets/ssw.svg")
        case "Fraktion DIE LINKE":
          return require("./../assets/linke.svg")
        case "Fraktion der CDU/CSU":
          return require("./../assets/cdu.svg")
        case "Petitionsausschuss":
          return require("./../assets/petitionsausschuss.svg")
      }

      return ""
    }
  }
}
</script>

