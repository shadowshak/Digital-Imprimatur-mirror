import React, { useState } from "react";
import {
  Box,
  AppBar,
  Toolbar,
  IconButton,
  Typography,
  Tabs,
  Tab,
  List,
  ListItem,
  Card,
  CardContent,
  CardActions,
  CardHeader,
  Grid,
  Drawer,
  Divider,
} from "@mui/material";
import { GrammarlyEditorPlugin } from "@grammarly/editor-sdk-react";
import CheckCircleOutlineIcon from '@mui/icons-material/CheckCircleOutline';
import { saveAs } from "file-saver";
import { demoText } from "./demo";
import "./document.css";

function Document({ role }) {
  const [text, setText] = useState("");
  const [showSuggestions, setShowSuggestions] = useState(false);

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

  const handleToggleSuggestions = () => {
    setShowSuggestions(!showSuggestions);
  };

  return (
    <>
      <AppBar>
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            Document Editing
          </Typography>
        </Toolbar>
      </AppBar>
      <Toolbar />
      <Grid container spacing={2}>
        <Grid item xs={12} md={8} sx={{ width: "65%" }}>
          <Card
            sx={{
              height: "100vh",
              display: "flex",
              flexDirection: "column",
              justifyContent: "space-between",
            }}
          >
            <CardContent>
              <h1>Grammarly API Editing</h1>
              <div className="container">
                <input type="file" id="file" onChange={handleFileUpload} />
                <GrammarlyEditorPlugin
                  className="grammar"
                  clientId="client_5HQzjrDKGyWSA1YHGTAQDx"
                  style={{ width: "900px"}}
                >
                  <textarea
                    id="text"
                    defaultValue={text || demoText.textarea}
                    rows={10}
                  />
                </GrammarlyEditorPlugin>
                <div className="actions">
                  <button onClick={handleSave}>Save Edited Document</button>
                </div>
              </div>
            </CardContent>
          </Card>
        </Grid>
        <Grid item xs={12} md={4} sx={{ width: "35%" }}>
          <Card
            sx={{
              height: "100vh",
              display: "flex",
              flexDirection: "column",
              justifyContent: "space-between",
            }}
          >
            <CardContent>
              <h1>Editing Suggestions</h1>  
              <Typography>
                Some text in the second card.
              </Typography>
              <Divider />
              <button onClick={handleToggleSuggestions}>
                Toggle Suggestions
              </button>
              <Drawer
                variant="persistent"
                anchor="right"
                open={showSuggestions}
              >
                <List>
                  <ListItem button>
                    <Typography>Some suggestion 1</Typography>
                  </ListItem>
                  <ListItem button>
                    <Typography>Some suggestion 2</Typography>
                    <IconButton>
                      <CheckCircleOutlineIcon />
                    </IconButton>
                  </ListItem>
                </List>
              </Drawer>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </>
  );
}

export default Document;