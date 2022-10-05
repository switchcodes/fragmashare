<script>
  import { invoke } from "@tauri-apps/api/tauri"
  import { onMount } from 'svelte';
  import { appDir } from '@tauri-apps/api/path';
  import ReceiveFiles from './ReceiveFiles.svelte';
  import SendFiles from './SendFiles.svelte';
  import { fileSet } from "./../stores.js";

  let operationView = null;

  async function createDir(){
    console.log(
      await invoke("create_settings_directory", { path: await appDir() })
    );
  }

  async function createServer(){
    await invoke("create_server"); 
  }

	onMount(async () => {
    // var ipAddressQrCodeCanvas = document.getElementById('ipAddressQrCodeCanvas')
    // QRCode.toCanvas(ipAddressQrCodeCanvas, await invoke("get_current_ip_address"), function (error) {
    //   if (error) console.error(error)
    //   console.log('success');
    // })
    // setInterval(() => {
    //   operationView =  [ReceiveFiles, SendFiles, Setup, null][Math.floor(Math.random() * 4)];
    // }, 2000);
    $fileSet = $fileSet;
    console.log("fileset in dashboard"+$fileSet);
	});

  // async function greet(){
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   greetMsg = await invoke("greet", { name })
  //   meetMsg = await invoke("meet");
  //   greetMsg += meetMsg;
  // }

  function handleMessage(){
    console.log("back button"); 
    operationView = null;
  }

</script>

<div id="mainView">
  <div id="welcome">
      {#if operationView == null}
        <h1>Home</h1>
        <p><i>Select one of the available operations below.</i></p>
      {:else if operationView == SendFiles}
         <h1>Send files to Smartphone</h1>
         <p><i>Drag the files you want to transfer anywhere on this program.</i></p>
      {:else if operationView == ReceiveFiles}
         <h1>Receive files from Smartphone</h1>
         <p><i>Select the files you want to transfer on your smartphone and scan the QR code.</i></p>
      {:else}
         <h1>Error</h1>
         <p><i>Something went wrong. Relaunch the program.</i></p>
      {/if}
  </div>

  {#if operationView == null}
    <div id="action">
      <input type="button" value="Send" on:click={()=>{operationView = SendFiles;}}>
      <input type="button" value="Receive" on:click={()=>{operationView = ReceiveFiles;}}>
    </div>
  {:else}
    <svelte:component on:message={handleMessage} this={operationView}/>
  {/if}

  <div id="miscInfo">
    <code>version 0.2</code>
  </div>

  <!-- <canvas id="ipAddressQrCodeCanvas"></canvas>

  <input type="button" value="Send now" on:click={sendFilesToBackend}>

  <input type="button" value="Create server" on:click={createServer}>

  <input type="button" value="Create directory" on:click={createDir}>

  {#each $fileSet as file}
      <p>{file}</p>
  {/each} -->
</div>

<style>

  #mainView{
    display: grid;
    grid-template-rows: 0.2fr 0.75fr 0.05fr;
    height: 100%;
    width: 100%;
    /* background: linear-gradient(180deg, #E1FAFF 0%, #D1F7FF 100%); */
    background-color: #e5e5f7;
    opacity: 1;
    background-image: radial-gradient(#9da2ff 0.6000000000000001px, #e5e5f7 0.6000000000000001px);
    background-size: 12px 12px;
  }

  #action{
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  #action input{
    position: relative;
    height: 15%;
    width: 35%;
    margin: 1rem 0rem 1rem 0rem;
    border: 2px solid var(--primaryColor);
    background-color: var(--primaryColor);
    color: #E1FAFF;
    font-size: 1.5rem;
    transition: transform 200ms;
    border-radius: 0.25rem;
  }

  #action input:hover{
    transform: scale(1.1);
    background-color: var(--primaryColorHighlight);
    border-color: var(--primaryColorHighlight);
    cursor: pointer;
  }

  #welcome{
    background-color: var(--primaryColor);
    color: #ffffff;
    width: calc(100%-1rem);
    padding-left: 1rem;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  #welcome > p{
    font-size: 1.25rem;
    margin-bottom: 0;
  }

  #miscInfo{
    width: 100%;
    background-color: var(--primaryColor);
    color: #E1FAFF;
    font-size: 1.25rem;
    display: flex;
    flex-direction: row;
    justify-content: flex-end;
    align-items: center;
  }

  #miscInfo > code{
    padding-right: 1rem;
  }
</style>

