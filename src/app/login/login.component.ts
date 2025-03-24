import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { AuthService } from '../services/auth.service';

@Component({
  selector: 'app-login',
  standalone: true,
  templateUrl: './login.component.html',
  styleUrl: './login.component.css',
  imports: [
    CommonModule,
    FormsModule
  ]
})
export class LoginComponent {
  email: string = '';
  senha: string = '';

  constructor(private authService: AuthService, private router: Router) { }

  async fazerLogin() {
    const sucess = await this.authService.login(this.email, this.senha);

    if (sucess) {
      alert("logado com sucesso")
      this.router.navigate(['/home']);
    } else {
      alert("login invalido")
    }
  }

  async forceHome() {

    console.log(this.authService.isLogged())
    this.router.navigate(['/home']);
  }
}
