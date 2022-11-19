<script>
    let objs = [
        {
            titel: "Protokoll der 67. Sitzung des 20. Deutschen Bundestages", 
            datum: "2022-11-11", 
            vorgaenge: [
                { 
                    titel: "Aktivitäten politischer Stiftungen in Thüringen mit Stand 30. September 2022", 
                    id: "293650", 
                    beratungsstand: "Noch nicht beantwortet", 
                    datum: "2022-11-18", 
                    initiative: ["CDU/CSU", "AfD"]
                },
                { 
                    titel: "Einfluss des Bundesministeriums der Finanzen auf Verordnungen aus dem Geschäftsbereich", 
                    id: "293646", 
                    beratungsstand: "Beantwortet", 
                    datum: "2022-11-18", 
                    initiative: ["Die LINKE"]
                }
            ]
        },
        {
            titel: "Protokoll der 65. Sitzung des 20. Deutschen Bundestages", 
            datum: "2022-11-11", 
            vorgaenge: [
                { 
                    titel: "Gesetz zur Abschaffung der Kostenheranziehung von jungen Menschen in der Kinder- und Jugendhilfe", 
                    id: "293650", 
                    beratungsstand: "Noch nicht beantwortet", 
                    datum: "2022-11-10", 
                    initiative: ["Die Grüne"]
                },
                { 
                    titel: "Gesetz zur Einführung eines Chancen-Aufenthaltsrechts", 
                    id: "293646", 
                    beratungsstand: "Beantwortet", 
                    datum: "2022-11-10", 
                    initiative: ["FDP"]
                }
            ]
        }
    ];
</script>

<main>
    <h1>Was letzte Gesetz?</h1>
    <!-- TODO: von allem oder von Allem? Besserer zusammenfassender Text -->
    <h2>Schon mal gefragt wofür die Parteien stimmen?<br/> Hier eine Auflistung von allem.</h2>
    {#each objs as obj}
        <div class="protokoll">
            <p>{obj.titel}</p>
            {obj.datum} <!-- TODO: better format -->

            <p>Vorgänge</p>
            {#each obj.vorgaenge as vorgang}
                <p>
                    {vorgang.id}<br/>
                    {vorgang.titel}<br/>
                    {vorgang.beratungsstand}<br/>
                    {vorgang.initiative}
                </p>
            {/each}
        </div>
    {/each}


</main>

<style>
    main {
        margin-left: 20%;
        margin-right: 20%;
    }

    .protokoll {
        padding: 10px;
        margin: 10px 0px;
        border: 1px solid #d5c4a1;
    }

    :global(body) {
        margin: 0;
        padding: 0;
        color: #3c3836;
        background-color: #fbf1c7;

        font-family: 'Roboto', sans-serif;
    }
</style>

