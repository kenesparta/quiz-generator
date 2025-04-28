import React from "react";
import { AppBar, Box, Divider, Drawer, Toolbar, Typography } from "@mui/material";
import { UserProfile } from "@shared/UserProfile/UserProfile";
import { SidebarMenu } from "@dashboard/SidebarMenu";

const drawerWidth = 240;

interface DashboardLayoutProps {
  children: React.ReactNode;
}

export const Dashboard = ({ children }: DashboardLayoutProps) => {
  return (
    <>
      <Box style={{ display: 'flex' }}>
        <AppBar
          position="fixed"
          sx={{ width: `calc(100% - ${drawerWidth}px)`, ml: `${drawerWidth}px`, boxShadow: 'none', backgroundColor: '#aaa' }}
        >
          <Toolbar>
            <Typography variant="h6" noWrap component="div">
              Dashboard
            </Typography>
            <UserProfile userName="Jane Smith" userAvatar="/path/to/avatar.jpg" />
          </Toolbar>
        </AppBar>

        <Drawer
          sx={{
            width: drawerWidth,
            flexShrink: 0,
            '& .MuiDrawer-paper': {
              width: drawerWidth,
              boxSizing: 'border-box',
            },
          }}
          variant="permanent"
          anchor="left"
        >
          <Toolbar>
            <Typography variant="h6" noWrap component="div">
              Menu
            </Typography>
          </Toolbar>
          <Divider/>
          <SidebarMenu/>
        </Drawer>

        <Box
          component="main"
          sx={{ flexGrow: 1, p: 3, width: { sm: `calc(100% - ${drawerWidth}px)` } }}
        >
          <Toolbar/>
          {children}
        </Box>
      </Box>
    </>
  )
}