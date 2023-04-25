import React, { useState } from "react";
import { GrammarlyEditorPlugin } from "@grammarly/editor-sdk-react";
import { saveAs } from "file-saver";
import { demoText } from "./demo";
import "./document.css";

function Document({ role }) {
  const [text, setText] = useState("");

  const handleFileUpload = (event) => {
    const file = event.target.files[0];
    const reader = new FileReader();
    reader.onload = (e) => {
      setText(e.target.result);
      document.querySelector("#text").value = e.target.result; // Update the textarea value
    };
    reader.readAsText(file);
  };
  
  const handleSave = async () => {
    const textToSave = document.querySelector("#text").value;
    const blob = new Blob([textToSave], { type: "text/plain;charset=utf-8" });
    saveAs(blob, "edited_document.txt");
  };

  return (
    <div className="container">
      <h1>Demo React App</h1>
      <h2>Upload a document</h2>
      <input type="file" id="file" onChange={handleFileUpload} />
      <GrammarlyEditorPlugin clientId="client_5HQzjrDKGyWSA1YHGTAQDx">
        <textarea id="text" defaultValue={text || demoText.textarea} rows={10} />
      </GrammarlyEditorPlugin>
      <div className="actions">
        <button onClick={handleSave}>Save</button>
      </div>
    </div>
  );
}

export default Document;
