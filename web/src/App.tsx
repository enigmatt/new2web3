import React, { useEffect, useState } from "react";
import logo from "./deadmandao-wiki-logo.png";
import loadedLogo from "./deadmandao-loaded.png";
import openSeaLogo from "./OpenSeaLogo.png";
import raribleLogo from "./RaribleLogo.png";
import "./App.scss";

import { Fluence } from "@fluencelabs/fluence";
import { krasnodar } from "@fluencelabs/fluence-network-environment";
import { get_first_opensea_page, get_first_rarible_page } from './_aqua/deadmandao';

const relayNode = krasnodar[0];

type Unpromise<T extends Promise<any>> = T extends Promise<infer U> ? U : never;

type RaribleResult = Unpromise<ReturnType<typeof get_first_rarible_page>>;
type OpenseaResult = Unpromise<ReturnType<typeof get_first_opensea_page>>;

function App() {
  {/* useState hooks for Fluence connectivity as well as status for OpenSea and Rarible queries */}
  const [isConnected, setIsConnected] = useState<boolean>(false);
  const [isRaribleLoaded, setIsRaribleLoaded] = useState<boolean>(false);
  const [isRaribleLoading, setIsRaribleLoading] = useState<boolean>(false);
  const [isOpenseaLoading, setIsOpenseaLoading] = useState<boolean>(false);
  const [isOpenseaLoaded, setIsOpenseaLoaded] = useState<boolean>(false);
  const [raribleResult, setRaribleResult] = useState<RaribleResult | null>(null);
  const [openseaResult, setOpenseaResult] = useState<OpenseaResult | null>(null);
  useEffect(() => {
    Fluence.start({ connectTo: relayNode.multiaddr })
      .then(() => setConnected())
      .catch((err) => console.log("Client initialization failed", err));
  }, [isConnected]);

  {/* setConnected does more than toggle the useState.  It also kicks off the Rarible and OpenSea initial loading. */}
  const setConnected = () => {
    setIsConnected(true);
    if (isRaribleLoading === false) {
      setIsRaribleLoading(true);
      doGetFirstRariblePage();
    }
    if (isOpenseaLoading === false) {
      setIsOpenseaLoading(true);
      doGetFirstOpenSeaPage();
    }
  }

  {/* Ansynchronous function to try and pull down the first set of Rarible NFTs via the Marine->Rust module */}
  const doGetFirstRariblePage = async () => {
    if (!isConnected) {
      return;
    }
    try {
      const res = await get_first_rarible_page();
      console.log("Fetched Rarible First Page", res);
      setRaribleResult(res);
      setIsRaribleLoaded(true);

    } catch (err: any) {
      console.log(err);
      setIsRaribleLoaded(false);
      setRaribleResult(null);
    }
    setIsRaribleLoading(false);
  }
  {/* Asynchronous function to try and pull down the first set of OpenSea NFTs via the Marin->Rust module */}
  const doGetFirstOpenSeaPage = async () => {
    if (!isConnected) {
      return;
    }
    try {
      const res = await get_first_opensea_page();
      console.log("Fetched OpenSea First Page", res);
      setOpenseaResult(res);
      setIsOpenseaLoaded(true);

    } catch (err: any) {
      console.log(err);
      setOpenseaResult(null);
      setIsOpenseaLoaded(false);
    }
    setIsOpenseaLoading(false);
  }

  {/* Clear associated flags before calligng doGetFirstRariblePage */}
  const clearFlagsAndLoadRarible = () => {
    setIsRaribleLoaded(false);
    setIsRaribleLoading(true);
    doGetFirstRariblePage();
  }
  {/* Clear associated flags before calligng doGetFirstOpenSeaPage */}
  const clearFlagsAndLoadOpenSea = () => {
    setIsOpenseaLoaded(false);
    setIsOpenseaLoading(true);
    doGetFirstOpenSeaPage();
  }

  {/* ReactJS's token return of the entire HTML page with embedded JavaScript-like functionality */}
  return (
    <div className="App">
      <header>
        <h1>DeadManDAO presents New2Web3</h1><img src={isConnected ? loadedLogo : logo} className="logo" alt="logo"/>
        <h3>An implementation of Fluence's Aqua and Marine to integrate with: 
        </h3>
        <a href="https://rarible.com/"><img src={raribleLogo} alt="Rarible Logo"/></a>
        <a href="https://opensea.io/"><img src={openSeaLogo} alt="OpenSea Logo"/></a>
      </header>

      <div className="content">
        <h3>{isConnected ? "Fluence Node Connected" : "Connecting to Fluence"}</h3>
        <div>

  {/* buttons for reloading Rarible and OpenSea - These services are heavily hit and frequently time out. */}
          <div className="row">
          <button onClick={() => clearFlagsAndLoadRarible()}>{isRaribleLoaded ? 'Rarible NFTs Loaded' : isRaribleLoading ? 'Getting Rarible NFTs' : 'Get Rarible NFTs'} </button>
          <button onClick={() => clearFlagsAndLoadOpenSea()}>{isOpenseaLoaded ? 'OpenSea NFTs Loaded' : isOpenseaLoading ? 'Getting OpenSea NFTs' : 'Get OpenSea NFTs'}</button>
          </div>
        </div>
{
  /* And now... to present the NFT images and brief descriptions... In a very hackish manner. */
  /* The correct way to do this would be to use ReactDOM.render and a map of the result set. */
}
        {raribleResult && (
          <div>
            <p className="success">Rarible returned {raribleResult.items.length} items</p>
            <table>
              <tbody>
                <tr >
                  <td>
                    <a href={"https://rarible.com/token/"+raribleResult.items[0].id.substring(raribleResult.items[0].blockchain.length+1)}>
                      <img className="nftImage" src={raribleResult.items[0].image_url} alt="thing"/>
                    </a>
                  </td>
                  <td>
                    <table className="uppity">
                      <tbody>
                        <tr><td className="bold">Description</td><td>{raribleResult.items[0].description}</td></tr>
                        <tr><td className="bold">Name</td><td>{raribleResult.items[0].name}</td></tr>
                      </tbody>
                    </table>
                  </td>
                </tr>
                <tr >
                  <td>
                  <a href={"https://rarible.com/token/"+raribleResult.items[1].id.substring(raribleResult.items[1].blockchain.length+1)}>
                      <img className="nftImage" src={raribleResult.items[1].image_url} alt="thing"/>
                    </a>
                  </td>
                  <td>
                    <table className="uppity">
                      <tbody>
                        <tr><td className="bold">Description</td><td>{raribleResult.items[1].description}</td></tr>
                        <tr><td className="bold">Name</td><td>{raribleResult.items[1].name}</td></tr>
                      </tbody>
                    </table>
                  </td>
                </tr>
                <tr >
                  <td>
                  <a href={"https://rarible.com/token/"+raribleResult.items[2].id.substring(raribleResult.items[2].blockchain.length+1)}>
                      <img className="nftImage" src={raribleResult.items[2].image_url} alt="thing"/>
                    </a>
                  </td>
                  <td>
                    <table className="uppity">
                      <tbody>
                        <tr><td className="bold">Description</td><td>{raribleResult.items[2].description}</td></tr>
                        <tr><td className="bold">Name</td><td>{raribleResult.items[2].name}</td></tr>
                      </tbody>
                    </table>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        )}
{/* And now... to present the OpenSea NFT images and brief descriptions... */}
        {openseaResult && (
          <div>
            <p className="success">OpenSea returned {openseaResult.items.length} items</p>
              <table>
                <tbody>
                  <tr >
                    <td>
                      <img className="nftImage" src={openseaResult.items[0].image_url} alt="thing"/>
                    </td>
                    <td>
                      <table className="uppity">
                        <tbody>
                        <tr><td className="bold">Name</td><td>{openseaResult.items[0].name}</td></tr>
                          <tr><td className="bold">Description</td><td>{openseaResult.items[0].description}</td></tr>
                        </tbody>
                      </table>
                    </td>
                  </tr>
                  <tr >
                    <td>
                      <img className="nftImage" src={openseaResult.items[1].image_url} alt="thing"/>
                    </td>
                    <td>
                      <table className="uppity">
                        <tbody>
                        <tr><td className="bold">Name</td><td>{openseaResult.items[1].name}</td></tr>
                          <tr><td className="bold">Description</td><td>{openseaResult.items[1].description}</td></tr>
                        </tbody>
                      </table>
                    </td>
                  </tr>
                  <tr >
                    <td>
                      <img className="nftImage" src={openseaResult.items[2].image_url} alt="thing"/>
                    </td>
                    <td>
                      <table className="uppity">
                        <tbody>
                          <tr><td className="bold">Name</td><td>{openseaResult.items[2].name}</td></tr>
                          <tr><td className="bold">Description</td><td>{openseaResult.items[2].description}</td></tr>
                        </tbody>
                      </table>
                    </td>
                  </tr>
                </tbody>
              </table>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
