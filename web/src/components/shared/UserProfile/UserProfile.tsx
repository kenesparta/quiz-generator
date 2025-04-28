"use client"

import React, { useState } from 'react';
import { Avatar, Box, IconButton, Menu, MenuItem, Tooltip, Typography } from '@mui/material';
import { AccountCircle, Logout, Settings } from '@mui/icons-material';

interface UserProfileProps {
  userName?: string;
  userAvatar?: string;
}

export const UserProfile: React.FC<UserProfileProps> = (
  {
    userName = "John Doe",
    userAvatar = undefined
  }
) => {
  const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);
  const open = Boolean(anchorEl);

  const handleClick = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = () => {
    setAnchorEl(null);
  };

  return (
    <Box sx={{ display: 'flex', alignItems: 'center' }}>
      <Tooltip title="Account settings">
        <IconButton
          onClick={handleClick}
          size="small"
          aria-controls={open ? 'account-menu' : undefined}
          aria-haspopup="true"
          aria-expanded={open ? 'true' : undefined}
        >
          <Avatar src={userAvatar} sx={{ width: 32, height: 32 }}>
            {!userAvatar && userName.charAt(0)}
          </Avatar>
        </IconButton>
      </Tooltip>

      <Typography
        variant="subtitle1"
        sx={{
          ml: 1,
          color: 'text.primary',
          display: { xs: 'none', sm: 'block' }
        }}
      >
        {userName}
      </Typography>

      <Menu
        anchorEl={anchorEl}
        id="account-menu"
        open={open}
        onClose={handleClose}
        onClick={handleClose}
        PaperProps={{
          elevation: 0,
          sx: {
            overflow: 'visible',
            filter: 'drop-shadow(0px 2px 8px rgba(0,0,0,0.32))',
            mt: 1.5,
            '& .MuiAvatar-root': {
              width: 32,
              height: 32,
              ml: -0.5,
              mr: 1,
            },
            '&:before': {
              content: '""',
              display: 'block',
              position: 'absolute',
              top: 0,
              right: 14,
              width: 10,
              height: 10,
              bgcolor: 'background.paper',
              transform: 'translateY(-50%) rotate(45deg)',
              zIndex: 0,
            },
          },
        }}
        transformOrigin={{ horizontal: 'right', vertical: 'top' }}
        anchorOrigin={{ horizontal: 'right', vertical: 'bottom' }}
      >
        <MenuItem onClick={handleClose}>
          <AccountCircle sx={{ mr: 2 }}/> Profile
        </MenuItem>
        <MenuItem onClick={handleClose}>
          <Settings sx={{ mr: 2 }}/> Settings
        </MenuItem>
        <MenuItem onClick={handleClose}>
          <Logout sx={{ mr: 2 }}/> Logout
        </MenuItem>
      </Menu>
    </Box>
  );
};