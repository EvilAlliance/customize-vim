<script async>
    import { appWindow } from '@tauri-apps/api/window'
    const Hide= "/public/TittleBar/Hide.png"; 
    const Minimize = "/public/TittleBar/Minimize.png"; 
    const Maximize = "/public/TittleBar/Maximize.png";
    const Close = "/public/TittleBar/Close.png";
    let ToggleMaximize = appWindow.isMaximized() ? Maximize : Minimize;
</script>

<style>
    header{
        height: 30px;
        background: #240046;
        user-select: none;
        display: flex;
        justify-content: flex-end;
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
    }

    header>div:nth-child(2){
        padding: 5px;
    }

    header>div{
        display: inline-flex;
        justify-content: center;
        align-items: center;
        height: 30px;
        aspect-ratio: 1;
        padding: 8px;
    }

    header>div:hover {
        background: #3C096C;
    }

    header>div>img{
        max-height: 100%;
        aspect-ratio: 1;
        object-fit:contain;
        filter: brightness(0) invert(0.3);
    }
</style>

<header data-tauri-drag-region>
    <div class="titlebar-button" on:click={()=>{appWindow.minimize()}}>
        <img
            src={Hide}
            alt="minimize"
        />
    </div>
    <div on:click={async()=>{
        appWindow.toggleMaximize();
        ToggleMaximize=await appWindow.isMaximized() ? Maximize : Minimize;
    }}>
        <img
            src={ToggleMaximize}
            alt="maximize"
        />
    </div>
    <div on:click={()=>{appWindow.close()}}>
        <img 
            src={Close}
            alt="close"
        />
    </div>
</header>
