<script>
    import TittleBar from "./Component/General/TittleBar.svelte";
    import { getFileTree } from "./Script/getFileTree.js";
    import Menu from "./Component/FilePicker/Menu.svelte";
</script>

<style>
    :global(*){
        box-sizing: border-box;
    }
    :global(div#app) {
        height: 100dvh;
        background:#10002B;
        margin: 0;
    }

    :global(body) {
        margin: 0;
    }

    section{
        display:flex;
        justify-content: center;
        align-items: center;
        height: 100%;
    }
</style>

<TittleBar/>
<section>
    <Menu/>
</section>
