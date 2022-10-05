<script>
  import Logo from "./../img/LogoNewWhite.svg"
</script>

<div id="loadingScreen">
  <div id="container">
    <div id="logoSection">
      <img id="logo" src={Logo} alt="Loading screen Logo">
    </div>
    <div id="infoSection">
      <p>Made by Bence Balázs</p>
    </div>
  </div>
</div>

<style>
  #infoSection{
    font-size: 1.5rem;
    color: #E1FAFF;
    display: flex;
    flex-direction: row;
    justify-content: center;
    place-items: center;
  }
  #logoSection{
    display: grid;
    place-content: center;
  }
  #logo{
    transform: scale(1.75);
  }
  #container{
    display: grid;
    grid-template-rows: 3fr 1fr;
    height: 100%;
    width: 100%;
  }
  #loadingScreen{
    height: 100%;
    width: 100%;
    /* background: linear-gradient(180deg, #E1FAFF 0%, #D1F7FF 100%); */
    background-color: var(--primaryColor);
  }
</style>