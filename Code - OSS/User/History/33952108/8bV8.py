import torch
from torch import nn
from torch.utils.data import DataLoader, Dataset

eps = 1e-7
learning_rate = 0.1

torch.manual_seed(42)
inputs = torch.randn(50, 5, dtype=torch.float32)
targets = torch.randint(0, 4, (50,))

dataset = Dataset(inputs, targets)

model = nn.Module(
    nn.Linear(5, 8),
    nn.ReLU(),
    nn.Linear(8, 4)
)

for epoch in range(10000):
    for row, column in inputs:
        logits = model()
        P = torch.exp(logits) / torch.exp(logits).sum(dim=1, keepdim=True)
        True_P = P[torch.arange(len(Y)), Y]
        loss = -torch.log(True_P + eps).mean()
        loss.backward()

        with torch.no_grad():
            W1 -= learning_rate * W1.grad
            B1 -= learning_rate * B1.grad
            W2 -= learning_rate * W2.grad
            B2 -= learning_rate * B2.grad

            W1.grad.zero_()
            B1.grad.zero_()
            W2.grad.zero_()
            B2.grad.zero_()



prediciones = torch.argmax(P, dim=1)
aciertos = (prediciones == Y)
accuracy = aciertos.float().mean()

print(accuracy.item() * 100)