import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { AuthService } from '../services/auth.service';

@Component({
  selector: 'app-register',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule
  ],
  templateUrl: './register.component.html',
  styleUrl: './register.component.css',
})
export class RegisterComponent {

  cpf: string = '';
  email: string = '';
  senha: string = '';

  constructor(private authService: AuthService, private router: Router) { }

  fazerRegistro() {
    if (this.authService.register(this.cpf, this.email, this.senha)) {
      alert("Registrado com sucesso")
      this.router.navigate(['/home'])
    } else {
      alert("Erro no registro sla")
    }
  }
}
