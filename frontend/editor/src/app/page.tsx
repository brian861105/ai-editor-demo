"use client";

import Editor from "@/components/editor";
import MainLayout from "@/components/layout/main-layout";
import { useState } from "react";

export default function Home() {
  const [saveStatus, setSaveStatus] = useState("Saved");

  const handleSave = () => {
    setSaveStatus("Saved");
  };

  return (
    <MainLayout saveStatus={saveStatus} onSave={handleSave}>
      <div className="flex flex-col">
        <Editor onSaveStatusChange={setSaveStatus} />
      </div>
    </MainLayout>
  );
}
