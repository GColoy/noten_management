export interface QuickAccessPiece {
  title: string;
  id: String;
}

export function getQuickAccessPieces(): QuickAccessPiece[] {
  return [
    { title: 'Pirates', id: '106DF3' },
    { title: 'Thunder and Lightning', id: '249FF7' },
    { title: 'Catskills', id: '35F589' },
  ]
}

export enum AccessRequestStatus {
  open,
  rejected,
  done,
}

export interface AccessRequest {
  Stück: string;
  User: string;
  Done: AccessRequestStatus;
}

export function getAccessRequests(): AccessRequest[] {
  return [
    { Stück: "Pirates", User: "Lukas", Done: AccessRequestStatus.open },
    { Stück: "Thunder and Lightning", User: "Max", Done: AccessRequestStatus.rejected },
    { Stück: "Catskills", User: "Moritz", Done: AccessRequestStatus.done },
  ]
}

export function getInboxCount() {
  return 23; 
}