import React, { useState } from "react";
import {
  Box,
  Button,
  AppBar,
  Dialog,
  DialogTitle,
  DialogActions,
  DialogContent,
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
  Fab,
  Slide,
  Snackbar,
} from "@mui/material";

import { useNavigate } from "react-router-dom";

import { Submission } from "./api";

import {
  Menu as MenuIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  Info as InfoIcon,
  Add as AddIcon,
  Visibility as VisibilityIcon,
} from "@mui/icons-material";
import DocumentSubmissionStepper from "../../components/SubmitDocument";
import IndividualDocument from "../../components/ViewDocumentProgress";
import axios from "axios";

const Transition = React.forwardRef(function Transition(props, ref) {
  return <Slide direction="left" ref={ref} {...props} />;
});

function AlertDialogSlide({ open, handleClose, page }) {
  const handleDialogClose = () => {
    console.log("handleClose called!");
    console.log(page.type.name.toString());
    console.log("nextstep");
    if ((page && page.type.name) === "DocumentSubmissionStepper") {
      console.log("Saving draft...");
      handleClose();
    } else {
      console.log("Closing...");
      handleClose();
    }
  };

  return (
    <Dialog
      open={open}
      TransitionComponent={Transition}
      onClose={handleDialogClose}
      keepMounted
      aria-describedby="alert-dialog-slide-description"
      PaperProps={{
        sx: {
          borderRadius: 0,
          height: "calc(100vh - 64px)",
          maxWidth: "100vw",
          position: "fixed",
          bottom: 0,
          right: 0,
          margin: 0,
          zIndex: 1299,
        },
      }}
    >
      <DialogContent sx={{ height: "calc(100vh - 64px)", overflow: "auto" }}>
        {page}
      </DialogContent>
    </Dialog>
  );
}

function Home({ role }) {
  const [submissions, setSubmissions] = React.useState([])

  const [value, setValue] = useState(1);
  const [open, setOpen] = useState(false);
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
  const [documentToDelete, setDocumentToDelete] = useState(null);
  const [page, setPage] = useState(null);
  const [deleteSnackbarOpen, setDeleteSnackbarOpen] = useState(false);
  const [draftSavedSnackbarOpen, setDraftSavedSnackbarOpen] = useState(false);

  const [ generation, setGeneration ] = useState(0);

  const handleChange = (event, newValue) => {
    setValue(newValue);
  };

  const handleNavigate = (page) => {
    setPage(page);
    setOpen(true);
  };

  const handleClose = () => {
    console.log("handleClose called!");
    console.log(page.type.name.toString());
    console.log("nextstep");
    if ((page && page.type.name) === "DocumentSubmissionStepper") {
      console.log("Saving draft...");
      setOpen(false);
      setDraftSavedSnackbarOpen(true);
      setPage(null);
      console.log(" ");
    } else {
      console.log("Closing...");
      setOpen(false);
      setPage(null);
      console.log(" ");
    }
  };

  const handleDelete = () => {
    handleDeleteDialogClose();
    setDeleteSnackbarOpen(true);

    setGeneration(generation + 1);
  };

  const handleDeleteDialogClose = () => {
    setDeleteDialogOpen(false);
  };

  const handleDeleteSnackbarClose = (event, reason) => {
    if (reason === "clickaway") {
      return;
    }

    setDeleteSnackbarOpen(false);
  };

  const handleDraftSavedSnackbarClose = (event, reason) => {
    if (reason === "clickaway") {
      return;
    }

    setDraftSavedSnackbarOpen(false);
  };

  const getSubmissions = async () => {
    const token = sessionStorage.getItem("token");

    const { data } = await axios.post("http://localhost:3001/user/submissions", {
      token
    });

    console.log(data);

    const responseSubmissions = data.submissions;
    const submissions = responseSubmissions.map(({
      id,
      meta: {
        name,
        author,
        description,
        creation,
        last_update,
        status,
      }
    }) => {
      return new Submission(id, name, author, description, new Date(creation), new Date(last_update), status)
    });

    setSubmissions(submissions)
  };

  React.useEffect(() => {
    try {
      getSubmissions();
    }
    catch(error) {
      console.log(JSON.stringify(error));
    }
  }, [generation]);

  return (
    <Box sx={{ backgroundColor: "#dee6ed", width: '100vw', height: '100vh' }}>
      <AppBar sx={{ zIndex: 1301, position: "sticky", top: 0 }}>
        <Toolbar>
          <IconButton
            size="large"
            edge="start"
            color="inherit"
            aria-label="menu"
            sx={{ mr: 2 }}
          >
            <MenuIcon />
          </IconButton>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            My Documents
          </Typography>
        </Toolbar>
      </AppBar>
      <Grid
        container
        direction="column"
        alignItems="center"
        justifyContent="center"
      >
        <Grid item>
          <Box>
            <Tabs
              value={value}
              onChange={handleChange}
              aria-label="Select Status Filter"
              sx={{ width: "100%" }}
            >
              <Tab label="Completed" sx={{ width: "250px" }} />
              <Tab label="In Progress" sx={{ width: "250px" }} />
              <Tab
                label={role === "publisher" ? "Needs Attention" : "Todo"}
                sx={{ width: "250px" }}
              />
            </Tabs>
            <Box sx={{ overflow: "auto" }}>
              {" "}
              <List>
                {
                submissions.map((submission) => {
                  return (
                  <ListItem>
                    <SubmissionCard submission={submission}
                                    role={role}
                                    handlers={{
                                      delete: handleDelete,
                                      navigate: handleNavigate,
                                    }}/>
                  </ListItem>
                  )
                })
                }
              </List>
            </Box>
          </Box>
        </Grid>
      </Grid>
      <Box sx={{ position: "fixed", bottom: "32px", right: "32px" }}>
        <Fab
          color="primary"
          aria-label="add"
          onClick={(event) =>
            handleNavigate(<DocumentSubmissionStepper onClose={handleClose} keepMounted />)
          }
        >
          <AddIcon />
        </Fab>
      </Box>
      <AlertDialogSlide open={open} handleClose={handleClose} page={page} />
      <Dialog open={deleteDialogOpen} onClose={handleDeleteDialogClose}>
        <DialogTitle>{`Delete ${documentToDelete}`}</DialogTitle>
        <DialogContent>
          <Typography>
            {`Are you sure you want to delete the document ${documentToDelete}?`}
          </Typography>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleDeleteDialogClose} color="primary">
            Cancel
          </Button>
          <Button onClick={handleDelete} color="primary" autoFocus>
            Delete
          </Button>
        </DialogActions>
      </Dialog>
      <Snackbar
        open={deleteSnackbarOpen}
        autoHideDuration={6000}
        onClose={handleDeleteSnackbarClose}
        message="Document Deleted"
        action={
          <Button color="primary" size="small" onClick={handleDelete}>
            Undo
          </Button>
        }
      />
      <Snackbar
        open={draftSavedSnackbarOpen}
        autoHideDuration={6000}
        onClose={handleDraftSavedSnackbarClose}
        message="Draft Saved"
      />
    </Box>
  );
}

function SubmissionCard({
  submission,
  role,
  handlers
}) {
  const navigate = useNavigate();

  const onDeleteClick = (e) => {
    e.handled = true;

    submission.delete()
    handlers.delete()
  };

  const onEditClick = () => {
    navigate(`/${role}/document`);
  }

  const onViewClick = () => {
    navigate(`/${role}/document`);
  }

  const onCardClick = () => {
    handlers.navigate(<IndividualDocument />)
  }

  return (
    <Card
      sx={{ minWidth: "100vh", cursor: "pointer" }}
      onClick={onCardClick}>
            <CardHeader
              action={
                <>
                  <IconButton sx={{ color: "#1976d2" }}
                              aria-label="Info"
                              onClick={onCardClick}>
                    <InfoIcon />
                  </IconButton>
                  <IconButton sx={{ color: "#1976d2" }}
                              aria-label="View"
                              onClick={onViewClick}>
                    <VisibilityIcon />

                    
                  </IconButton>
                  <IconButton sx={{ color: "#1976d2" }}
                              aria-label="Edit"
                              onClick={onEditClick}>
                    <EditIcon />
                  </IconButton>

                  {role === "publisher" && (
                    <IconButton aria-label="Delete" onClick={onDeleteClick}>
                      <DeleteIcon sx={{ color: "#1976d2" }} />
                    </IconButton>
                  )}
                </>
              }
              title={submission.title}
            />
            <CardContent>
              <Typography sx={{ mt: -4 }} color="text.secondary">
                {submission.author}
              </Typography>
              <Typography sx={{ mb: 1.5 }} color="text.secondary">
                {submission.creation.toLocaleDateString()}
              </Typography>
              <Typography
                variant="body2"
                color="text.secondary"
                sx={{ fontStyle: "italic" }}
              >
                {submission.description}
              </Typography>
            </CardContent>
            <CardActions></CardActions>
          </Card>)
}

export default Home;
