"use client"

import React from 'react';

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
  return (
    <>
      User Profile
      <p>
        {userName}
      </p>
      <p>
        {userAvatar}
      </p>
    </>
  );
};
