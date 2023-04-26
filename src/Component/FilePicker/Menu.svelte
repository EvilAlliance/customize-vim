<script>
</script>
<style>
    main {
        display: grid;
        grid-template-rows: 90% 1fr;
        grid-template-columns: 65% 1fr;
        width:80dvw;
        height: 75dvh;
        row-gap:10px;
        column-gap:10px;
    }
    main>*{
        border-radius:10px;
        border: 2px solid #5A189A;
    }
    main>header{
        grid-row: 1/2;
        grid-column: 1/2;
    }
    main>section{
        grid-row: 1/2;
        grid-column: 2/3;
    }
    main>footer{
        grid-row: 2/3;
        grid-column:1/3;
    }

</style>

<main>
    <header>
    </header>
    <section>
    </section>
    <footer>
    </footer>
</main>
