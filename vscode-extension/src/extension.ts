import * as vscode from 'vscode';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

let statusBarItem: vscode.StatusBarItem;
let diagnosticCollection: vscode.DiagnosticCollection;

function getBinaryPath(): string {
    return vscode.workspace.getConfiguration('agentml').get<string>('binaryPath', 'agentml');
}

async function runAgentml(args: string[], cwd?: string): Promise<{ stdout: string; stderr: string }> {
    const binary = getBinaryPath();
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
    return execAsync(`"${binary}" ${args.join(' ')}`, { cwd: cwd || workspaceFolder });
}

function showOutput(output: string, title: string) {
    const channel = vscode.window.createOutputChannel(`AgentML: ${title}`);
    channel.appendLine(output);
    channel.show();
}

async function validateContract() {
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
    if (!workspaceFolder) {
        vscode.window.showWarningMessage('AgentML: No workspace folder open');
        return;
    }

    try {
        const { stdout } = await runAgentml(['validate', 'AGENT.agent']);
        statusBarItem.text = '$(shield) AgentML: Valid';
        statusBarItem.backgroundColor = undefined;
        diagnosticCollection.clear();
        vscode.window.showInformationMessage('AgentML: Contract is valid ✓');
    } catch (error: any) {
        const output = error.stdout || error.stderr || error.message;
        statusBarItem.text = '$(error) AgentML: Invalid';
        statusBarItem.backgroundColor = new vscode.ThemeColor('statusBarItem.errorBackground');

        // Parse errors into diagnostics if AGENT.agent is open
        const doc = await vscode.workspace.openTextDocument(
            vscode.Uri.joinPath(vscode.Uri.file(workspaceFolder), 'AGENT.agent')
        );
        const diagnostics: vscode.Diagnostic[] = [];
        const lines = output.split('\n');
        for (const line of lines) {
            const match = line.match(/line (\d+)/i);
            if (match) {
                const lineNum = parseInt(match[1], 10) - 1;
                const range = new vscode.Range(lineNum, 0, lineNum, 200);
                diagnostics.push(new vscode.Diagnostic(range, line, vscode.DiagnosticSeverity.Error));
            }
        }
        if (diagnostics.length > 0) {
            diagnosticCollection.set(doc.uri, diagnostics);
        }

        showOutput(output, 'Validation Failed');
        vscode.window.showErrorMessage('AgentML: Contract validation failed. See output for details.');
    }
}

export function activate(context: vscode.ExtensionContext) {
    diagnosticCollection = vscode.languages.createDiagnosticCollection('agentml');
    context.subscriptions.push(diagnosticCollection);

    statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = '$(shield) AgentML';
    statusBarItem.tooltip = 'AgentML contract status';
    statusBarItem.command = 'agentml.validate';
    statusBarItem.show();
    context.subscriptions.push(statusBarItem);

    context.subscriptions.push(
        vscode.commands.registerCommand('agentml.init', async () => {
            const template = await vscode.window.showQuickPick(
                ['generic', 'rust-cli', 'nextjs-app', 'react-app', 'python-package', 'node-package', 'go-cli', 'django-app'],
                { placeHolder: 'Select a project template' }
            );
            if (!template) return;
            try {
                const { stdout } = await runAgentml(['init', '--template', template]);
                showOutput(stdout, 'Init');
                vscode.window.showInformationMessage(`AgentML: Initialized with ${template} template ✓`);
                validateContract();
            } catch (error: any) {
                vscode.window.showErrorMessage(`AgentML init failed: ${error.message}`);
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('agentml.validate', validateContract)
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('agentml.brief', async () => {
            try {
                const { stdout } = await runAgentml(['brief']);
                showOutput(stdout, 'Brief');
            } catch (error: any) {
                vscode.window.showErrorMessage(`AgentML brief failed: ${error.message}`);
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('agentml.diff', async () => {
            try {
                const { stdout } = await runAgentml(['diff']);
                showOutput(stdout, 'Diff Audit');
            } catch (error: any) {
                showOutput(error.stdout || error.message, 'Diff Audit');
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('agentml.doctor', async () => {
            try {
                const { stdout } = await runAgentml(['doctor']);
                showOutput(stdout, 'Doctor');
            } catch (error: any) {
                vscode.window.showErrorMessage(`AgentML doctor failed: ${error.message}`);
            }
        })
    );

    // Validate on save
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument((doc) => {
            const validateOnSave = vscode.workspace.getConfiguration('agentml').get<boolean>('validateOnSave', true);
            if (validateOnSave && doc.fileName.endsWith('AGENT.agent')) {
                validateContract();
            }
        })
    );

    // Initial validation if AGENT.agent exists
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
    if (workspaceFolder) {
        const fs = require('fs');
        const path = require('path');
        if (fs.existsSync(path.join(workspaceFolder, 'AGENT.agent'))) {
            validateContract();
        }
    }
}

export function deactivate() {}
