<script>
    import TittleBar from "./Component/General/TittleBar.svelte";
    import { getFileTree } from "./Script/getFileTree.js";
    import Menu from "./Component/FilePicker/Menu.svelte";
</script>

<main>
    <TittleBar />
    <section>
        <Menu />
    </section>
</main>

<style>
    :global(*) {
        box-sizing: border-box;
    }
    :global(div#app) {
        height: 100dvh;
        background: #10002b;
        margin: 0;
    }
    :global(body) {
        margin: 0;
    }
    main {
        height: 100%;
        display: grid;
        grid-template-rows: 30px 1fr;
    }
    section {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100%;
    }
</style>