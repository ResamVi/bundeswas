<template>
  <div class="relative max-w-5xl mx-auto pt-20 sm:pt-24 lg:pt-32">
    <h1 class="text-gray-900 font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center">
      Was letzte Gesetz?
    </h1>
    <p class="mt-6 text-lg text-gray-600 text-center max-w-3xl mx-auto">
      Diese Seite ist (einführende Erklärung hier).<br/>
      Angezeigt wird die letzte Plenarsitzung und alle Vorgänge die stattgefunden haben.<br/>
      Um mehr zu erfahren schaue <a href="https://cli.vuejs.org" target="_blank" rel="noopener">hier</a>.
    </p>
  </div>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 md:px-8 mt-20 sm:mt-24 lg:mt-32">
    <div class="hello">
      <Plenarprotokoll v-for="(prot, index) in protokolle" :protokoll="prot" :key="index"></Plenarprotokoll>
    </div>
  </div>
</template>

<script>
import Plenarprotokoll from './components/Plenarprotokoll.vue'

export default {
  name: 'App',
  components: {
    Plenarprotokoll
  },
  data() {
    return {
      protokolle: [],
    }
  },
  mounted() {
    fetch('http://localhost:1337/plenarprotokolle')
        .then(response => response.json())
        .then(data => this.protokolle = data);
  },
}
</script>

<style>
#app {
  margin-top: 60px;
}
p {
  font-family: 'Inter', sans-serif;
}
</style>
