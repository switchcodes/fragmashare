<script>
  import { createEventDispatcher, onDestroy, onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { get_current_component } from "svelte/internal";
  import { listen } from "@tauri-apps/api/event";
  import { fileSet } from "./../stores.js";
  import QRCode from "qrcode";

  const component = get_current_component();
  const svelteDispatch = createEventDispatcher();
  let unlistenFileDrop, unlistenCountdown, unlistenCoutdownClosure;
  let buttonDisable = false;
  let countdownProgress = 15;

  const dispatch = (name, detail) => {
    svelteDispatch(name, detail);
    component.dispatchEvent &&
      component.dispatchEvent(new CustomEvent(name, { detail }));
  };

  function handleBack() {
    dispatch("message");
    console.log("hello");
  }

  onMount(async () => {
    unlistenFileDrop = await listen("tauri://file-drop", async (event) => {
      let payload = event.payload;
      // console.log(event)
      console.log("fileset start drop event:" + $fileSet);
      try {
        payload.forEach((filePath) => {
          // let filePathSplit = filePath.split("\\");
          // filePath = filePathSplit[filePathSplit.length - 1]
          $fileSet = Array.from(new Set([...$fileSet, filePath]));
        });
        $fileSet = $fileSet;
        // console.log("fileset after import of event:"+$fileSet);
        //await invoke("print_selected_files", { "filePathStringArray": event.payload });
        // console.log($fileSet);
        console.log("fileset end drop event:" + $fileSet);
      } catch (error) {
        console.log(error);
      }
    });
    unlistenCountdown = await listen(
      "frgshr://connection-validity",
      (event) => {
        countdownProgress = event.payload.message;
        console.log(event.payload);
      }
    );

    unlistenCoutdownClosure = await listen(
      "frgshr://connection-validity-automatic-closure",
      (event) => {
        countdownProgress = 15;
        generateQrCode();
        console.log(event.payload);
      }
    );
  });

  onDestroy(() => {
    unlistenFileDrop();
    unlistenCountdown();
    unlistenCoutdownClosure();
  });

  function clearFileSet() {
    $fileSet = [];
    console.log("fileset after clear: " + $fileSet);
  }

  async function sendFilesToBackend() {
    await invoke("print_selected_files", { filePathStringArray: $fileSet });
    $fileSet = [];
  }

  async function generateQrCode() {
    // disable buttons back and clear
    buttonDisable = true;
    // generate QRcode
    let ipAddress = await invoke("get_current_ip_address");
    let ipAddressQrCodeCanvas = document.getElementById(
      "ipAddressQrCodeCanvas"
    );
    let secret = ipAddress + "_" + (await invoke("create_secret"));
    QRCode.toCanvas(ipAddressQrCodeCanvas, secret, function (error) {
      if (error) console.error(error);
      console.log("success");
    });
    // send backend the paths and wait for completion
    await invoke("print_selected_files", {
      filePathStringArray: $fileSet,
      secret: secret,
    });
    await invoke("create_server");
  }
</script>

<div id="setup">
  <div id="backArea">
    <input
      class:buttonDisable
      disabled={buttonDisable}
      type="button"
      value="Back to home"
      on:click={handleBack}
    />
    <input
      class:buttonDisable
      disabled={buttonDisable}
      type="button"
      value="Clear import list"
      on:click={clearFileSet}
    />
    {#if buttonDisable}
      <input
        id="cancelQr"
        type="button"
        value="Cancel"
        on:click={async () => {
          await invoke("terminate_server");
          buttonDisable = !buttonDisable;
        }}
      />
    {:else if $fileSet.length > 0}
      <input
        id="generateQr"
        type="button"
        value="Generate QRcode"
        on:click={generateQrCode}
      />
    {/if}
  </div>
  <div id="actionArea">
    <div id="dragList">
      <div id="dragListHelper">
        {#each $fileSet as file}
          <div id="fileComponent">
            <code>{file.split("\\")[file.split("\\").length - 1]}</code>
          </div>
        {/each}
      </div>
    </div>
    <div id="arrowCentering">
      {#if buttonDisable}
        <div class="arrow" />
      {/if}
    </div>
    <div id="sendArea">
        {#if $fileSet.length > 0 && buttonDisable}
        <div id="sendAreaStyling">
        <code id="qrInfo">Each QRcode lasts 15 seconds!</code>
          <div id="centerCountdown">
            <div
              role="progressbar"
              aria-valuemin={0}
              aria-valuenow={countdownProgress}
              aria-valuemax={15}
              style={"--value:"+countdownProgress}
            />
          </div>
          <canvas id="ipAddressQrCodeCanvas" />
        </div>
        {/if}
    </div>
  </div>
</div>

<style>
  #setup {
    height: 100%;
    width: 100%;
    color: var(--primaryColor);
    display: grid;
    grid-template-rows: 1fr 6fr;
  }

  #centerCountdown{
    display: grid;
    place-items: center;
  }

  #qrInfo{
    background-color: white;
    padding: 0.5rem;
    font-size: 1rem;
  }

  div[role="progressbar"] {
    --size: 6rem;
    --fg: var(--primaryColor);
    --bg: white;
    --pgPercentage: var(--value);
    animation: growProgressBar 0.1s 1 forwards;
    width: var(--size);
    height: var(--size);
    border-radius: 50%;
    display: grid;
    place-items: center;
    background: radial-gradient(
        closest-side,
        white 75%,
        transparent 0 99.9%,
        white 0
      ),
      conic-gradient(var(--fg) calc(var(--pgPercentage) * 6.66%), var(--bg) 0);
    font-family: Helvetica, Arial, sans-serif;
    font-size: calc(var(--size) / 5);
    color: var(--fg);
  }

  div[role="progressbar"]::before {
    counter-reset: percentage var(--value);
    content: counter(percentage) " sec";
  }

  @keyframes growProgressBar {
    0%,
    33% {
      --pgPercentage: 0;
    }
    100% {
      --pgPercentage: var(--value);
    }
  }

  @property --pgPercentage {
    syntax: "<number>";
    inherits: false;
    initial-value: 0;
  }

  #sendArea {
    height: 100%;
    width: 100%;
  }

  #sendAreaStyling{
    height: calc(100% - 1.2rem);
    width: calc(100% - 2.2rem);
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 2rem;
    margin: 0rem 1rem 1rem 1rem;
    border: 2px solid var(--primaryColor);
    border-radius: 0.25rem;
    background-color: white;
  }

  #fileComponent {
    padding: 0.25rem 0.75rem 0.25rem 0.75rem;
    font-size: 1.25rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  #dragList {
    position: relative;
    margin: 0rem 1rem 1rem 1rem;
    border: 2px solid var(--primaryColor);
    border-radius: 0.25rem;
    background-color: #ffffff;
  }

  #dragListHelper {
    position: absolute;
    height: 100%;
    width: 100%;
    overflow-y: auto;
  }

  /* width */
  #dragListHelper::-webkit-scrollbar {
    width: 0.5rem;
  }

  /* Track */
  #dragListHelper::-webkit-scrollbar-track {
    background: #f1f1f1;
  }

  /* Handle */
  #dragListHelper::-webkit-scrollbar-thumb {
    background: var(--primaryColor);
  }

  /* Handle on hover */
  #dragListHelper::-webkit-scrollbar-thumb:hover {
    background: var(--primaryColorHighlight);
  }

  #arrowCentering {
    display: grid;
    place-content: center;
  }

  #actionArea {
    display: grid;
    grid-template-columns: 5fr 1fr 4fr;
  }

  #backArea {
    height: calc(100% - 2rem);
    width: calc(100% - 2rem);
  }

  #backArea input {
    position: relative;
    height: fit-content;
    width: fit-content;
    padding: 0.25rem 1rem 0.25rem 1rem;
    margin: 1rem;
    border: 2px solid var(--primaryColor);
    background-color: var(--primaryColor);
    color: #e1faff;
    font-size: 1rem;
    transition: transform 200ms;
    border-radius: 0.25rem;
  }

  #backArea input:hover {
    transform: scale(1.1);
    background-color: var(--primaryColorHighlight);
    border-color: var(--primaryColorHighlight);
    cursor: pointer;
  }

  .arrow {
    width: 50px;
    height: 30px;
    display: grid;
    overflow: hidden;
  }
  .arrow:before,
  .arrow:after {
    content: "";
    grid-area: 1/1;
    background: currentColor;
    clip-path: polygon(
      0 10px,
      calc(100% - 15px) 10px,
      calc(100% - 15px) 0,
      100% 50%,
      calc(100% - 15px) 100%,
      calc(100% - 15px) calc(100% - 10px),
      0 calc(100% - 10px)
    );
    animation: a5 1.5s infinite;
    transform: translate(calc(0% + var(--s, 0%)));
  }

  .arrow:after {
    --s: -100%;
  }

  @keyframes a5 {
    80%,
    100% {
      transform: translate(calc(100% + var(--s, 0%)));
    }
  }

  .buttonDisable {
    text-decoration: line-through;
    opacity: 0.4;
  }

  .buttonDisable:hover {
    transform: scale(1) !important;
    background-color: var(--primaryColor) !important;
    border-color: var(--primaryColor) !important;
    cursor: not-allowed !important;
  }
</style>
