import React from "react";
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
} from "@mui/material";
import MenuIcon from "@mui/icons-material/Menu";
import VisibilityIcon from "@mui/icons-material/Visibility";
import EditIcon from "@mui/icons-material/Edit";
import DeleteIcon from "@mui/icons-material/Delete";
import { Link } from "react-router-dom";

function Home({ role }) {
  const [value, setValue] = React.useState(1);

  const handleChange = (event, newValue) => {
    setValue(newValue);
  };

  return (
    <>
      <AppBar>
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
      <Toolbar />
      <Grid
        container
        direction="column"
        alignItems="center"
        justifyContent="center"
        sx={{
          backgroundColor: "#dee6ed",
        }}
      >
        <Grid item>
          <Box>
            <Tabs
              value={value}
              onChange={handleChange}
              aria-label="Select Status Filter"
            >
              <Tab label="Completed" sx={{ width: "250px" }} />
              <Tab label="In Progress" sx={{ width: "250px" }} />
              <Tab
                label={role === "publisher" ? "Needs Attention" : "Todo"}
                sx={{ width: "250px" }}
              />
            </Tabs>
            <Box sx={{ overflow: "auto" }}>
              <List>
                <ListItem>
                <Card sx={{ minWidth: "100vh" }}>
                    <CardHeader
                      action={
                        <>
                          <IconButton aria-label="View" component={Link} to="/publisher/document-viewer">
                            <VisibilityIcon sx={{ color: "#1976d2" }} />
                          </IconButton>
                          <IconButton aria-label="Edit" component={Link} to="/reviewer/document">
                            <EditIcon sx={{ color: "#1976d2" }} />
                          </IconButton>
                          {role === "publisher" && (
                            <IconButton aria-label="Delete">
                              <DeleteIcon sx={{ color: "#1976d2" }} />
                            </IconButton>
                          )}
                        </>
                      }
                      title="Book or Publication Title"
                    />
                    <CardContent>
                      <Typography sx={{ mt: -4 }} color="text.secondary">
                        Author Name
                      </Typography>
                      <Typography sx={{ mb: 1.5 }} color="text.secondary">
                        Date Published
                      </Typography>
                      <Typography
                        variant="body2"
                        color="text.secondary"
                        sx={{ fontStyle: "italic" }}
                      >
                        Description
                      </Typography>
                    </CardContent>
                    <CardActions></CardActions>
                  </Card>
                </ListItem>
                <ListItem>
                  <Card sx={{ minWidth: "100vh" }}>
                    <CardHeader
                      action={
                        <>
                          <IconButton aria-label="View">
                            <VisibilityIcon sx={{ color: "#1976d2" }} />
                          </IconButton>
                          <IconButton aria-label="Edit">
                            <EditIcon sx={{ color: "#1976d2" }} />
                          </IconButton>
                          {role === "publisher" && (
                            <IconButton aria-label="Delete">
                              <DeleteIcon sx={{ color: "#1976d2" }} />
                            </IconButton>
                          )}
                        </>
                      }
                      title="Book or Publication Title"
                    />
                    <CardContent>
                      <Typography sx={{ mt: -4 }} color="text.secondary">
                        Author Name
                      </Typography>
                      <Typography sx={{ mb: 1.5 }} color="text.secondary">
                        Date Published
                      </Typography>
                      <Typography
                        variant="body2"
                        color="text.secondary"
                        sx={{ fontStyle: "italic" }}
                      >
                        Description
                      </Typography>
                    </CardContent>
                    <CardActions></CardActions>
                  </Card>
                </ListItem>
                <ListItem>
                  <Card sx={{ minWidth: "100vh" }}>
                    <CardHeader
                      action={
                        <>
                          <IconButton aria-label="View">
                            <VisibilityIcon sx={{ color: "#1976d2" }} />
                          </IconButton>
                          <IconButton aria-label="Edit">
                            <EditIcon sx={{ color: "#1976d2" }} />
                          </IconButton>
                          {role === "publisher" && (
                            <IconButton aria-label="Delete">
                              <DeleteIcon sx={{ color: "#1976d2" }} />
                            </IconButton>
                          )}
                        </>
                      }
                      title="Book or Publication Title"
                    />
                    <CardContent>
                      <Typography sx={{ mt: -4 }} color="text.secondary">
                        Author Name
                      </Typography>
                      <Typography sx={{ mb: 1.5 }} color="text.secondary">
                        Date Published
                      </Typography>
                      <Typography
                        variant="body2"
                        color="text.secondary"
                        sx={{ fontStyle: "italic" }}
                      >
                        Description
                      </Typography>
                    </CardContent>
                    <CardActions></CardActions>
                  </Card>
                </ListItem>
                <ListItem>
                  <Card sx={{ minWidth: "100vh" }}>
                    <CardHeader
                      action={
                        <>
                          <IconButton aria-label="View">
                            <VisibilityIcon sx={{ color: "#1976d2" }} />
                          </IconButton>
                          <IconButton aria-label="Edit">
                            <EditIcon sx={{ color: "#1976d2" }} />
                          </IconButton>
                          {role === "publisher" && (
                            <IconButton aria-label="Delete">
                              <DeleteIcon sx={{ color: "#1976d2" }} />
                            </IconButton>
                          )}
                        </>
                      }
                      title="Book or Publication Title"
                    />
                    <CardContent>
                      <Typography sx={{ mt: -4 }} color="text.secondary">
                        Author Name
                      </Typography>
                      <Typography sx={{ mb: 1.5 }} color="text.secondary">
                        Date Published
                      </Typography>
                      <Typography
                        variant="body2"
                        color="text.secondary"
                        sx={{ fontStyle: "italic" }}
                      >
                        Description
                      </Typography>
                    </CardContent>
                    <CardActions></CardActions>
                  </Card>
                </ListItem>
                <ListItem>
                  <Card sx={{ minWidth: "100vh" }}>
                    <CardHeader
                      action={
                        <>
                          <IconButton aria-label="View">
                            <VisibilityIcon sx={{ color: "#1976d2" }} />
                          </IconButton>
                          <IconButton aria-label="Edit">
                            <EditIcon sx={{ color: "#1976d2" }} />
                          </IconButton>
                          {role === "publisher" && (
                            <IconButton aria-label="Delete">
                              <DeleteIcon sx={{ color: "#1976d2" }} />
                            </IconButton>
                          )}
                        </>
                      }
                      title="Book or Publication Title"
                    />
                    <CardContent>
                      <Typography sx={{ mt: -4 }} color="text.secondary">
                        Author Name
                      </Typography>
                      <Typography sx={{ mb: 1.5 }} color="text.secondary">
                        Date Published
                      </Typography>
                      <Typography
                        variant="body2"
                        color="text.secondary"
                        sx={{ fontStyle: "italic" }}
                      >
                        Description
                      </Typography>
                    </CardContent>
                    <CardActions></CardActions>
                  </Card>
                </ListItem>
              </List>
            </Box>
          </Box>
        </Grid>
      </Grid>
    </>
  );
}

export default Home;
