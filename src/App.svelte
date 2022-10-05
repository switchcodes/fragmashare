<script>
  import Dashboard from './lib/Dashboard.svelte';
  import LoadingScreen from './lib/LoadingScreen.svelte';
  import Setup from './lib/Setup.svelte';
  import { onMount } from 'svelte';
  import { config } from './stores.js';
  import { invoke } from '@tauri-apps/api/tauri';
  import { appDir } from '@tauri-apps/api/path';
  import { appWindow } from '@tauri-apps/api/window'

  let view;

  // 0. load loadingscreen
  // 1. check for config file
  // 2. if no file then create a dir and a file and init setup (tour, empty store and settings customization)
  // 3. if yes then load settings within the file into the store
  // 4. wait for store property "init" then load dashboard
  // config schema: identifier_setupComplete_colorSchema_optionalName

  onMount(async () => {
    document
    .getElementById('titlebar-minimize')
    .addEventListener('click', () => appWindow.minimize())
  document
    .getElementById('titlebar-maximize')
    .addEventListener('click', () => appWindow.toggleMaximize())
  document
    .getElementById('titlebar-close')
    .addEventListener('click', () => appWindow.close())
    view = LoadingScreen;
    setTimeout(()=>{init();},2500)
  });

  async function init(){
    let configCheck = await invoke("check_for_config", { path: await appDir() });
    console.log(await appDir());
    console.log(configCheck); // errorhandling?
    if(configCheck.split("_")[0] == "#1#") {
      let tempConfig = [];
      configCheck.split("_").forEach(element => {
        tempConfig.push(element)
      });
      $config.settings = tempConfig;
      $config.initState = false;
    }
    if($config.initState){
      view = Setup;
    } else {
      view = Dashboard;
    }
  }
</script>

<main class="container"  on:dragover={(ev) => { ev.preventDefault() }}>
  <div class="titlebar">
    <div class="titlebar-button" id="titlebar-minimize">
      <img
        src="https://api.iconify.design/mdi:window-minimize.svg?color=white"
        alt="minimize"
      />
    </div>
    <div class="titlebar-button" id="titlebar-maximize">
      <img
        src="https://api.iconify.design/mdi:window-maximize.svg?color=white"
        alt="maximize"
      />
    </div>
    <div class="titlebar-button" id="titlebar-close">
      <img src="https://api.iconify.design/mdi:close.svg?color=white" alt="close" />
    </div>
  </div>
  <div class="dragRegion" data-tauri-drag-region></div>
  <svelte:component this={view}/>
</main>

<style>
  :root{
    --primaryColor: #00599C;
    --primaryColorHighlight: #1979c2;
    --primaryColorDarken:  #013e6d;
  }

  .container {
    position: relative;
    height: 100%;
    width: 100%;
  }

  .dragRegion{
    position: absolute;
    top: 0;
    right: 0;
    left: 0;
    height: 15px;
    background: var(--primaryColorDarken);
    z-index: 5;
  }

  .dragRegion:hover{
    cursor: grab;
  }

  .titlebar {
    position: absolute;
    top: 0;
    right: 0;
    background: var(--primaryColorDarken);
    user-select: none;
    display: flex;
    justify-content: flex-end;
    z-index: 10;
  }
  .titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 30px;
    height: 100%;
    padding: 0.5rem 0rem 0.5rem 0rem;
  }
  .titlebar-button > img{
    stroke: white;
  }
  .titlebar-button:hover {
    background: var(--primaryColor);
    cursor: pointer;
  }
</style>