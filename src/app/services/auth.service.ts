import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  private isAuthenticated: boolean;

  constructor() {
    const storedAuth = localStorage.getItem('isAuthenticated');
    this.isAuthenticated
      = storedAuth === 'true';
  }

  changeAuthBool(new_auth: boolean) {
    if (new_auth) {
      this.isAuthenticated = true;
      localStorage.setItem('isAuthenticated', 'true');
    } else {
      this.isAuthenticated = false;
      localStorage.removeItem('isAuthenticated');
    }
  }

  login(email: string, senha: string): boolean {
    if (email === 'admin@admin.com' && senha === '1234') {
      this.changeAuthBool(true);
      return true;
    }
    return false;
  }

  register(cpf: string, email: string, senha: string): boolean {
    // Tratamento dos dados, vou criar um novo service especificamente para isso 

    // placeholder temporario

    if (cpf != '' && email != '' && senha != '') {
      this.isAuthenticated = true;
      return true
    }

    return false;
  }

  logout() {
    this.changeAuthBool(false);
  }

  isLogged(): boolean {
    return this.isAuthenticated;
  }
}

