<script>
    import { onMount } from "svelte";
    import { data } from "../store.js";

    onMount(async () => {
        fetch("http://localhost:3000/")
            .then(response => response.json())
            .then(json => {
                data.set(json);
            }).catch(err => {
                console.log(err);
                return [];
            })
    });

    let allObjects;

    data.subscribe(value => {
        allObjects = value;
    });

    let map = {
        "Gesetzgebung": "laws.png"
    }

</script>

<main>
    <h1>Wasletztegesetz?</h1>
    <!-- TODO: von allem oder von Allem? Besserer zusammenfassender Text -->
    <h2>Schon mal gefragt wofür die Parteien stimmen?<br/> Hier eine Auflistung von allem.</h2>
    {#each allObjects as obj}
        <div class="protokoll">
            <p class="underline">{obj.titel} <a href={obj.pdf_url}>[#]</a></p>
            {obj.datum} <!-- TODO: nach rechts -->

            <p>Vorgänge</p>
            {#each obj.vorgaenge as vorgang}
                <div class="card" style="display: flex; gap: 20px;">
                    <img src={map[vorgang.vorgangstyp]} alt={vorgang.vorgangstyp} style="object-fit: contain;">
                    <div>{vorgang.titel}</div>
                    <div>{vorgang.beratungsstand}</div>
                    <div>{vorgang.initiative}</div>
                </div>
            {/each}
        </div>
    {/each}

    <a href="https://www.flaticon.com/free-icons/law" title="law icons">Law icons created by monkik - Flaticon</a>
</main>

<style>
    /* TODO: local */
    @import url(//db.onlinewebfonts.com/c/245d97414b360a07ca9c274af2dc38ef?family=NeuzeitGro-Bol);
    @font-face {
        font-family: "NeuzeitGro-Bol";
        src: url("//db.onlinewebfonts.com/t/245d97414b360a07ca9c274af2dc38ef.eot");
        src: url("//db.onlinewebfonts.com/t/245d97414b360a07ca9c274af2dc38ef.eot?#iefix") format("embedded-opentype"), 
        url("//db.onlinewebfonts.com/t/245d97414b360a07ca9c274af2dc38ef.woff2") format("woff2"), 
        url("//db.onlinewebfonts.com/t/245d97414b360a07ca9c274af2dc38ef.woff") format("woff"),
        url("//db.onlinewebfonts.com/t/245d97414b360a07ca9c274af2dc38ef.ttf") format("truetype"),
        url("//db.onlinewebfonts.com/t/245d97414b360a07ca9c274af2dc38ef.svg#NeuzeitGro-Bol") format("svg"); 
    }

    main {
        margin-left: 20%;
        margin-right: 20%;
    }

    .protokoll {
        padding: 10px;
        margin: 10px 0px;
        /* border: 1px solid #d5c4a1; */
    }

    .underline{
        border-bottom: 1px solid #000000;
        width: 100%;
        display: block;
        font-family: "NeuzeitGro-Bol";
    }

    :global(body) {
        margin: 0;
        padding: 0;
        color: #3c3836;
        background-color: #f5f5f5;

        font-family: 'Roboto', sans-serif;
    }

    .card {
        box-shadow: 0 0 20px rgba(0,0,0,0.4);
        border-radius: 5px;
        margin: 50px 20px 20px 20px;
        padding: 20px;
    }

    h1 {
        font-family: "NeuzeitGro-Bol";
        text-transform: uppercase;
        font-size: 60px;
        text-align: center;
    }

    h2 {
        text-align: center;
    }

</style>

